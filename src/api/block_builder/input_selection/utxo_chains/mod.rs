// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! input selection for utxo chains

use std::{collections::HashSet, ops::Deref, str::FromStr};

use bee_api_types::responses::OutputResponse;
use bee_block::{
    address::{Address, AliasAddress, NftAddress},
    output::{
        dto::OutputDto, feature::SenderFeature, AliasOutput, AliasOutputBuilder, FoundryOutputBuilder,
        NativeTokensBuilder, NftOutput, NftOutputBuilder, Output, OutputId, Rent, RentStructure,
    },
    payload::transaction::TransactionId,
};

use super::types::AccumulatedOutputAmounts;
use crate::{
    api::input_selection::{get_accumulated_output_amounts, output_contains_address, sdr_not_expired},
    secret::types::InputSigningData,
    Client, Result,
};
mod automatic;
mod validation;
pub(crate) use validation::check_utxo_chain_inputs;

// Select required alias, nft and foundry outputs. When the amount of alias or nft outputs is > the minimum required
// storage deposit and burning is not allowed, they will be added in the input and also to the outputs, but there just
// with the minimum required storage deposit as amount, so the difference gets available. Sender features will be
// removed.
#[allow(clippy::too_many_arguments)]
pub(crate) fn select_utxo_chain_inputs(
    selected_inputs: &mut Vec<InputSigningData>,
    selected_inputs_output_ids: &mut HashSet<OutputId>,
    selected_input_amount: &mut u64,
    selected_input_native_tokens: &mut NativeTokensBuilder,
    outputs: &mut Vec<Output>,
    required: &mut AccumulatedOutputAmounts,
    utxo_chain_inputs: &Vec<&InputSigningData>,
    allow_burning: bool,
    current_time: u32,
    rent_structure: &RentStructure,
) -> crate::Result<()> {
    // if an output is required as input, but we don't want to burn/destroy it, we have to add it as output again.
    // We track here for which outputs we did that, to prevent doing it multiple times.
    let mut added_output_for_input_signing_data = HashSet::new();

    // Add existing selected inputs we added for sender and issuer features before
    for input_signing_data in selected_inputs.iter() {
        // Add inputs to outputs if not already there, so they don't get burned
        if !allow_burning {
            add_output_for_input(input_signing_data, rent_structure, outputs)?;
        }
        added_output_for_input_signing_data.insert(input_signing_data.output_id()?);
    }

    loop {
        let outputs_len_beginning = outputs.len();

        for input_signing_data in utxo_chain_inputs {
            let output_id = input_signing_data.output_id()?;

            // Skip inputs where we already added the required output.
            if added_output_for_input_signing_data.contains(&output_id) {
                continue;
            }

            let minimum_required_storage_deposit = input_signing_data.output.rent_cost(rent_structure);

            match &input_signing_data.output {
                Output::Nft(nft_input) => {
                    let nft_id = nft_input.nft_id().or_from_output_id(output_id);
                    // or if an output contains an nft output with the same nft id
                    let is_required = outputs.iter().any(|output| {
                        if let Output::Nft(nft_output) = output {
                            nft_id == *nft_output.nft_id()
                        } else {
                            false
                        }
                    });
                    if !is_required && !allow_burning {
                        let nft_address = Address::Nft(NftAddress::new(nft_id));
                        let nft_required_in_unlock_condition = outputs.iter().any(|output| {
                            // check if alias address is in unlock condition
                            output_contains_address(output, output_id, &nft_address, current_time)
                        });

                        // Don't add if it doesn't give us any amount or native tokens
                        if !nft_required_in_unlock_condition
                            && input_signing_data.output.amount() == minimum_required_storage_deposit
                            && nft_input.native_tokens().is_empty()
                        {
                            continue;
                        }
                        // Remove potential SenderFeature because we don't need it and don't want to check it again
                        let filtered_features = nft_input
                            .features()
                            .iter()
                            .cloned()
                            .filter(|feature| feature.kind() != SenderFeature::KIND);
                        // else add output to outputs with minimum_required_storage_deposit
                        let new_output = NftOutputBuilder::from(nft_input)
                            .with_nft_id(nft_input.nft_id().or_from_output_id(output_id))
                            .with_amount(minimum_required_storage_deposit)?
                            // replace with filtered features
                            .with_features(filtered_features)
                            .finish_output()?;
                        outputs.push(new_output);
                        added_output_for_input_signing_data.insert(output_id);
                    }
                }
                Output::Alias(alias_input) => {
                    let alias_id = alias_input.alias_id().or_from_output_id(output_id);
                    // Don't add if output has not the same AliasId, so we don't burn it
                    if !outputs.iter().any(|output| {
                        if let Output::Alias(alias_output) = output {
                            alias_id == *alias_output.alias_id()
                        } else {
                            false
                        }
                    }) && !allow_burning
                    {
                        let alias_address = Address::Alias(AliasAddress::new(alias_id));
                        let alias_required_in_unlock_condition = outputs.iter().any(|output| {
                            // check if alias address is in unlock condition
                            output_contains_address(output, output_id, &alias_address, current_time)
                        });

                        // Don't add if it doesn't give us any amount or native tokens
                        if !alias_required_in_unlock_condition
                            && input_signing_data.output.amount() == minimum_required_storage_deposit
                            && alias_input.native_tokens().is_empty()
                        {
                            continue;
                        }

                        // Remove potential SenderFeature because we don't need it and don't want to check it again
                        let filtered_features = alias_input
                            .features()
                            .iter()
                            .cloned()
                            .filter(|feature| feature.kind() != SenderFeature::KIND);
                        // else add output to outputs with minimum_required_storage_deposit
                        let new_output = AliasOutputBuilder::from(alias_input)
                            .with_alias_id(alias_input.alias_id().or_from_output_id(output_id))
                            .with_state_index(alias_input.state_index() + 1)
                            .with_amount(minimum_required_storage_deposit)?
                            // replace with filtered features
                            .with_features(filtered_features)
                            .finish_output()?;
                        outputs.push(new_output);
                        added_output_for_input_signing_data.insert(output_id);
                    }
                }
                Output::Foundry(foundry_input) => {
                    // Don't add if output has not the same FoundryId, so we don't burn it
                    if !outputs.iter().any(|output| {
                        if let Output::Foundry(foundry_output) = output {
                            foundry_input.id() == foundry_output.id()
                        } else {
                            false
                        }
                    }) && !allow_burning
                    {
                        // Don't add if it doesn't give us any amount or native tokens
                        if input_signing_data.output.amount() == minimum_required_storage_deposit
                            && foundry_input.native_tokens().is_empty()
                        {
                            continue;
                        }
                        // else add output to outputs with minimum_required_storage_deposit
                        let new_output = FoundryOutputBuilder::from(foundry_input)
                            .with_amount(minimum_required_storage_deposit)?
                            .finish_output()?;
                        outputs.push(new_output);
                        added_output_for_input_signing_data.insert(output_id);
                    }
                }
                _ => {}
            }

            // Don't add inputs multiple times
            if !selected_inputs_output_ids.contains(&output_id) {
                let output = &input_signing_data.output;
                *selected_input_amount += output.amount();

                if let Some(output_native_tokens) = output.native_tokens() {
                    selected_input_native_tokens.add_native_tokens(output_native_tokens.clone())?;
                }

                if let Some(sdr) = sdr_not_expired(output, current_time) {
                    // add sdr to required amount, because we have to send it back
                    required.amount += sdr.amount();
                }

                selected_inputs.push(input_signing_data.deref().clone());
                selected_inputs_output_ids.insert(output_id);

                // Updated required value with possible new input
                let input_outputs = selected_inputs.iter().map(|i| &i.output);
                *required = get_accumulated_output_amounts(&input_outputs, outputs.iter())?;
            }
        }

        // If the output amount changed, we added at least one new one output, if not, we can break, because we added
        // all required ones.
        if outputs_len_beginning == outputs.len() {
            break;
        }
    }
    Ok(())
}

/// Get recursively owned alias and nft outputs and add them to the utxo_chains
pub(crate) async fn get_alias_and_nft_outputs_recursively(
    client: &Client,
    utxo_chains: &mut Vec<(Address, OutputResponse)>,
) -> Result<()> {
    log::debug!("[get_alias_and_nft_outputs_recursively]");
    let current_time = client.get_time_checked().await?;

    let mut processed_alias_nft_addresses = std::collections::HashSet::new();

    // Add addresses for alias and nft outputs we already have
    for (_unlock_address, output_response) in utxo_chains.iter() {
        let output_id = OutputId::new(
            TransactionId::from_str(&output_response.metadata.transaction_id)?,
            output_response.metadata.output_index,
        )?;

        match Output::try_from(&output_response.output)? {
            Output::Alias(alias_output) => {
                processed_alias_nft_addresses.insert(Address::Alias(AliasAddress::new(
                    alias_output.alias_id().or_from_output_id(output_id),
                )));
            }
            Output::Nft(nft_output) => {
                processed_alias_nft_addresses.insert(Address::Nft(NftAddress::new(
                    nft_output.nft_id().or_from_output_id(output_id),
                )));
            }
            _ => {}
        }
    }

    let mut processed_utxo_chains = Vec::new();

    // Make the outputs response optional, because we don't know it yet for new required outputs
    let mut utxo_chain_optional_response: Vec<(Address, Option<OutputResponse>)> =
        utxo_chains.iter_mut().map(|(a, o)| (*a, Some(o.clone()))).collect();

    // Get alias or nft addresses when needed or just add the input again
    while let Some((unlock_address, output_response)) = utxo_chain_optional_response.pop() {
        // Don't request outputs for addresses where we already have the output
        if processed_alias_nft_addresses.insert(unlock_address) {
            match unlock_address {
                Address::Alias(address) => {
                    let output_id = client.alias_output_id(*address.alias_id()).await?;
                    let output_response = client.get_output(&output_id).await?;
                    if let OutputDto::Alias(alias_output_dto) = &output_response.output {
                        let alias_output = AliasOutput::try_from(alias_output_dto)?;
                        // State transition if we add them to inputs
                        let alias_unlock_address = alias_output.state_controller_address();
                        // Add address to unprocessed_alias_nft_addresses so we get the required output there
                        // also
                        if alias_unlock_address.is_alias() || alias_unlock_address.is_nft() {
                            utxo_chain_optional_response.push((*alias_unlock_address, None));
                        }
                        processed_utxo_chains.push((*alias_unlock_address, output_response));
                    }
                }
                Address::Nft(address) => {
                    let output_id = client.nft_output_id(*address.nft_id()).await?;
                    let output_response = client.get_output(&output_id).await?;
                    if let OutputDto::Nft(nft_output) = &output_response.output {
                        let nft_output = NftOutput::try_from(nft_output)?;
                        let unlock_address = nft_output
                            .unlock_conditions()
                            .locked_address(nft_output.address(), current_time);
                        // Add address to unprocessed_alias_nft_addresses so we get the required output there also
                        if unlock_address.is_alias() || unlock_address.is_nft() {
                            utxo_chain_optional_response.push((*unlock_address, None));
                        }
                        processed_utxo_chains.push((*unlock_address, output_response));
                    }
                }
                _ => {}
            }
        }

        // Add if the output_response is available
        if let Some(output_response) = output_response {
            processed_utxo_chains.push((unlock_address, output_response));
        }
    }

    *utxo_chains = processed_utxo_chains;

    Ok(())
}

// If we have an input that is an alias, nft or foundry output and we don't want to burn it, then we need to add it to
// the output side. This function will do that with the minimum required storage deposit and potential sender feature
// removed.
fn add_output_for_input(
    input_signing_data: &InputSigningData,
    rent_structure: &RentStructure,
    outputs: &mut Vec<Output>,
) -> crate::Result<()> {
    let output_id = input_signing_data.output_id()?;
    let minimum_required_storage_deposit = input_signing_data.output.rent_cost(rent_structure);

    match &input_signing_data.output {
        Output::Nft(nft_input) => {
            let nft_id = nft_input.nft_id().or_from_output_id(output_id);
            // Don't add if nft output is already present in the outputs.
            if !outputs.iter().any(|output| {
                if let Output::Nft(nft_output) = output {
                    nft_id == *nft_output.nft_id()
                } else {
                    false
                }
            }) {
                // Remove potential SenderFeature because we don't need it and don't want to check it again
                let filtered_features = nft_input
                    .features()
                    .iter()
                    .cloned()
                    .filter(|feature| feature.kind() != SenderFeature::KIND);
                // add output to outputs with minimum_required_storage_deposit
                let new_output = NftOutputBuilder::from(nft_input)
                    .with_nft_id(nft_input.nft_id().or_from_output_id(output_id))
                    .with_amount(minimum_required_storage_deposit)?
                    // replace with filtered features
                    .with_features(filtered_features)
                    .finish_output()?;
                outputs.push(new_output);
            }
        }
        Output::Alias(alias_input) => {
            let alias_id = alias_input.alias_id().or_from_output_id(output_id);
            // Don't add if alias output is already present in the outputs.
            if !outputs.iter().any(|output| {
                if let Output::Alias(alias_output) = output {
                    alias_id == *alias_output.alias_id()
                } else {
                    false
                }
            }) {
                // Remove potential SenderFeature because we don't need it and don't want to check it again
                let filtered_features = alias_input
                    .features()
                    .iter()
                    .cloned()
                    .filter(|feature| feature.kind() != SenderFeature::KIND);
                // else add output to outputs with minimum_required_storage_deposit
                let new_output = AliasOutputBuilder::from(alias_input)
                    .with_alias_id(alias_input.alias_id().or_from_output_id(output_id))
                    .with_state_index(alias_input.state_index() + 1)
                    .with_amount(minimum_required_storage_deposit)?
                    // replace with filtered features
                    .with_features(filtered_features)
                    .finish_output()?;
                outputs.push(new_output);
            }
        }
        Output::Foundry(foundry_input) => {
            // Don't add if foundry output is already present in the outputs.
            if !outputs.iter().any(|output| {
                if let Output::Foundry(foundry_output) = output {
                    foundry_input.id() == foundry_output.id()
                } else {
                    false
                }
            }) {
                // else add output to outputs with minimum_required_storage_deposit
                let new_output = FoundryOutputBuilder::from(foundry_input)
                    .with_amount(minimum_required_storage_deposit)?
                    .finish_output()?;
                outputs.push(new_output);
            }
        }
        _ => {}
    }
    Ok(())
}
