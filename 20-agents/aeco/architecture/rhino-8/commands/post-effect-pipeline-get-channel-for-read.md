# post-effect-pipeline-get-channel-for-read

Lifecycle: single

Get a channel for reading. A post effect will use this to get channel data as input to             its process. Output will be written to new channel(s). \see GetChannelForWrite()             This method returns the current state of the channel at this stage in the pipeline.             If the first post effect calls this, it will get the actual frame buffer channel.             Subsequent post effects will get the data left behind by the previous post effect.             A post effect calls GetChannelForRead() to get its input and GetChannelForWrite()             to get the object to which it will write its output. Even when the same channel id             is specified, these are separate, unconnected objects.
