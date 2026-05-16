# post-effect-pipeline-get-channel-for-write

Lifecycle: single

Get a channel for writing. A post effect will use this to get channel(s) to write the output of its             processing to. Input will usually come from existing channels, although a post effect is free to read             its own output channels if needed. See GetChannelForRead()             You are allowed to create one new channel with the same identifier as an existing channel,             in which case IChannel::Commit() will replace the existing channel with the new one in the pipeline.
