// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { MessageHandler } from './MessageHandler';
import type {
    AccountBalance,
    Address,
    AccountSyncOptions,
    AccountMeta,
    OutputsToCollect,
    OutputData,
    Transaction,
    NativeTokenOptions,
    TransactionOptions,
    NftOptions,
    AddressWithAmount,
    AddressWithMicroAmount,
    AddressNativeTokens,
    AddressNftId,
    AddressGenerationOptions,
    AddressWithUnspentOutputs,
    TransactionResult,
    PreparedTransactionData,
    Output,
} from '../types';
import type { SignedTransactionEssence } from '../types/signedTransactionEssence';

export class Account {
    meta: AccountMeta;
    private messageHandler: MessageHandler;

    constructor(accountMeta: AccountMeta, messageHandler: MessageHandler) {
        this.meta = accountMeta;
        this.messageHandler = messageHandler;
    }

    async collectOutputs(outputIds: string[]): Promise<TransactionResult[]> {
        const resp = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'CollectOutputs',
                data: {
                    outputIdsToCollect: outputIds,
                },
            },
        );
        return JSON.parse(resp).payload;
    }

    getAlias(): string {
        return this.meta.alias;
    }

    async setAlias(alias: string): Promise<void> {
        await this.messageHandler.callAccountMethod(this.meta.index, {
            name: 'SetAlias',
            data: {
                alias
            }
        })
    }

    async getBalance(): Promise<AccountBalance> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'GetBalance',
            },
        );

        return JSON.parse(response).payload;
    }

    async getOutput(outputId: string): Promise<OutputData> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'GetOutput',
                data: {
                    outputId,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async getOutputsWithAdditionalUnlockConditions(
        outputs: OutputsToCollect,
    ): Promise<string[]> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'GetOutputsWithAdditionalUnlockConditions',
                data: {
                    outputsToCollect: outputs,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async getTransaction(transactionId: string): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'GetTransaction',
                data: {
                    transactionId,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async listAddresses(): Promise<Address[]> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'ListAddresses',
            },
        );

        return JSON.parse(response).payload;
    }

    async listAddressesWithUnspentOutputs(): Promise<
        AddressWithUnspentOutputs[]
    > {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'ListAddressesWithUnspentOutputs',
            },
        );

        return JSON.parse(response).payload;
    }

    async listOutputs(): Promise<OutputData[]> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'ListOutputs',
            },
        );

        return JSON.parse(response).payload;
    }

    async listUnspentOutputs(): Promise<OutputData[]> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'ListUnspentOutputs',
            },
        );

        return JSON.parse(response).payload;
    }

    async listPendingTransactions(): Promise<Transaction[]> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'ListPendingTransactions',
            },
        );
        return JSON.parse(response).payload;
    }

    async listTransactions(): Promise<Transaction[]> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'ListTransactions',
            },
        );

        return JSON.parse(response).payload;
    }

    async sync(options?: AccountSyncOptions): Promise<void> {
        await this.messageHandler.callAccountMethod(this.meta.index, {
            name: 'SyncAccount',
            data: options ?? {},
        });
    }

    async generateAddress(
        options?: AddressGenerationOptions,
    ): Promise<Address> {
        const addresses = await this.generateAddresses(1, options);
        return addresses[0];
    }

    async generateAddresses(
        amount: number,
        options?: AddressGenerationOptions,
    ): Promise<Address[]> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'GenerateAddresses',
                data: {
                    amount,
                    options,
                },
            },
        );

        return JSON.parse(response).payload;
    }

    async mintNativeToken(
        nativeTokenOptions: NativeTokenOptions,
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionResult> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'MintNativeToken',
                data: {
                    nativeTokenOptions: nativeTokenOptions,
                    options: transactionOptions,
                },
            },
        );

        return JSON.parse(response).payload;
    }

    async mintNfts(
        nftsOptions: NftOptions[],
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionResult> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'MintNfts',
                data: {
                    nftsOptions,
                    options: transactionOptions,
                },
            },
        );

        return JSON.parse(response).payload;
    }

    async prepareTransaction(
        outputs: Output[],
        options?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'PrepareTransaction',
                data: {
                    outputs,
                    options,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async prepareMintNfts(
        nftOptions: NftOptions[],
        options?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'PrepareMintNfts',
                data: {
                    nftOptions,
                    options,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async prepareSendAmount(
        addressWithAmount: AddressWithAmount[],
        options?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'PrepareSendAmount',
                data: {
                    addressWithAmount,
                    options,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async prepareSendMicroTransaction(
        addressWithMicroAmounts: AddressWithMicroAmount[],
        options?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'PrepareSendMicroTransaction',
                data: {
                    addressWithMicroAmounts,
                    options,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async prepareSendNativeToken(
        addressNativeTokens: AddressNativeTokens[],
        options?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'PrepareSendNativeToken',
                data: {
                    addressNativeTokens,
                    options,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async prepareSendNft(
        addressNftIds: AddressNftId[],
        options?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'PrepareSendNft',
                data: {
                    addressNftIds,
                    options,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async sendAmount(
        addressesWithAmount: AddressWithAmount[],
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionResult> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'SendAmount',
                data: {
                    addressWithAmount: addressesWithAmount,
                    options: transactionOptions,
                },
            },
        );

        return JSON.parse(response).payload;
    }

    async sendMicroTransaction(
        addressesWithMicroAmount: AddressWithMicroAmount[],
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionResult> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'SendMicroTransaction',
                data: {
                    addressWithMicroAmount: addressesWithMicroAmount,
                    options: transactionOptions,
                },
            },
        );

        return JSON.parse(response).payload;
    }

    async sendNativeTokens(
        addressesNativeTokens: AddressNativeTokens[],
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionResult> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'SendNativeTokens',
                data: {
                    addressNativeTokens: addressesNativeTokens,
                    options: transactionOptions,
                },
            },
        );

        return JSON.parse(response).payload;
    }

    async sendNft(
        addressesAndNftIds: AddressNftId[],
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionResult> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'SendNft',
                data: {
                    addressNftIds: addressesAndNftIds,
                    options: transactionOptions,
                },
            },
        );

        return JSON.parse(response).payload;
    }

    async SendTransaction(
        outputs: Output[],
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionResult> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'SendTransaction',
                data: {
                    outputs,
                    options: transactionOptions,
                },
            },
        );

        return JSON.parse(response).payload;
    }

    async tryCollectOutputs(
        outputsToCollect: OutputsToCollect,
    ): Promise<TransactionResult[]> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'TryCollectOutputs',
                data: {
                    outputsToCollect,
                },
            },
        );

        return JSON.parse(response).payload;
    }

    async signTransactionEssence(preparedTransactionData: PreparedTransactionData): Promise<SignedTransactionEssence> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'SignTransactionEssence',
                data: {
                    preparedTransactionData,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async submitAndStoreTransaction(signedTransactionData: SignedTransactionEssence): Promise<TransactionResult> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'SubmitAndStoreTransaction',
                data: {
                    signedTransactionData,
                },
            },
        );
        return JSON.parse(response).payload;
    }
}