export declare interface NodeInfo { }
export declare interface Message { }
export declare interface MessageMetadata { }
export declare interface OutputMetadata { }
export declare interface MilestoneMetadata { }
export declare interface BrokerOptions {
  automaticDisconnect: bool
  // timeout in milliseconds
  timeout: number
}

export declare class ClientBuilder {
  node(url: string): ClientBuilder
  nodes(urls: string[]): ClientBuilder
  quorumSize(size: number): ClientBuilder
  quorumThreshold(threshold: number): ClientBuilder
  brokerOptions(options: BrokerOptions): ClientBuilder
  build(): Client
}

export declare class ValueTransactionSender {
  path(bip32path: string): ValueTransactionSender
  index(index: number): ValueTransactionSender
  output(address: string, value: number): ValueTransactionSender
  send(): Promise<string>
}

export declare class UnspentAddressGetter {
  path(bip32path: string): ValueTransactionSender
  index(index: number): ValueTransactionSender
  send(): Promise<[string, number]>
}

export declare class Client {
  subscriber(): TopicSubscriber
  send(seed: string): ValueTransactionSender
  getUnspentAddress(seed: string): UnspentAddressGetter
  getInfo(): Promise<NodeInfo>
  getTips(): Promise<[string, string]>
  postMessage(message: Message): Promise<string>
  getMessage(): MessageFinder
  getOutput(outputId: string): Promise<OutputMetadata>
  findOutputs(outputIds: string[], addresses: string[]): Promise<OutputMetadata[]>
  getAddressOutputs(address: string): Promise<string[]>
  getAddressBalance(address: string): Promise<number>
  getMilestone(index: number): Promise<MilestoneMetadata>
  retry(messageId: string): Promise<Message>
  reattach(messageId: string): Promise<Message>
  promote(messageId: string): Promise<Message>
}

export declare class MessageFinder {
  index(index: string): Promise<string[]>
  data(messageId: string): Promise<Message>
  raw(messageId: string): Promise<string>
  children(messageId: string): Promise<string[]>
  metadata(messageId: string): Promise<MessageMetadata>
}

export declare type Callback = (err: any, data: any) => void

export declare class TopicSubscriber {
  topic(topic: string): TopicSubscriber
  topics(topic: string[]): TopicSubscriber
  subscribe(cb: Callback): TopicSubscriber
  unsubscribe(cb: Callback): TopicSubscriber
}
