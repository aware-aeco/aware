# render-window-add-wireframe-channel

Lifecycle: single

This method sets the frame buffer size and adds all the necessary wireframe channels automatically.             It also creates the wireframe channel data automatically so that your renderer doesn't have to.             You typically call this method only when your renderer does not support wireframe rendering itself.             If you call this method, then you should not add any wireframe channels returned by GetRenderChannels().             If your renderer is capable of rendering the wireframe channels itself, you should not call this method.             Instead, you must make sure you add the wireframe channels if GetRenderChannels() requests them.             See IRhRdkRenderWindow::GetRenderChannels().             After the wires are rendered, the wireframe post effects will composite them into the final rendered image.             Note: This method should really be called AddWireframeChannels(). [SDK_UNFREEZE] */
