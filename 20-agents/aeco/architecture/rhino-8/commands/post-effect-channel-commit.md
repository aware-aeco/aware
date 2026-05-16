# post-effect-channel-commit

Lifecycle: single

Commit changes to the channel so that those changes can be used by subsequent post effects in the chain.             Only valid for channels that were obtained by calling GetChannelForWrite().             If the channel has the same id as an existing channel, the existing channel will be replaced by             the new one. If the existing channel was created by a previous post effect in the chain, it will be deleted.             Changes to channels that are not commited simply get ignored.             Note: This call merely sets a flag. The process is deferred until after the post effect has finished executing.
