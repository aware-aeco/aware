---
name: rhino-common-rhino-runtime-rhino-accounts
description: API reference for namespace Rhino.Runtime.RhinoAccounts from RhinoCommon.dll
---

# Rhino.Runtime.RhinoAccounts

- **IOAuth2Token**
  - Represents an OAuth2 Token that can be used for authorization purposes.
- **IOpenIDConnectToken**
  - This class represents an OpenIDConnect token issued from an OpenID provider. The token is immutable.
- **IRhinoAccountsManager**
  - Performs various Rhino Accounts-related tasks.
- **ProgressState**
  - Describes the state of progress.
- **RhinoAccoountsProgressInfo**
  - This class is designed to convey the progress of an asynchronous operation through .NET's  interface.              It can be used by callers of such tasks to relay the tasks' progress, as well as useful metadata information that may be of interest.
- **RhinoAccountsAuthTokenMismatchException**
  - Exception thrown when the currently logged in user is different from the newly logged in user.
- **RhinoAccountsCannotListenException**
  - Exception thrown when there is no port available on the machine for Rhino to listen for Rhino Accounts' response.
- **RhinoAccountsException**
  - Base exception for all Rhino Accounts operations.
- **RhinoAccountsGroup**
  - Represents a Rhino Accounts group.
- **RhinoAccountsInvalidResponseException**
  - Exception thrown when the response returned by Rhino Accounts is not valid.
- **RhinoAccountsInvalidStateException**
  - Occurs when the state returned by the Rhino Accounts server is not the same as the one sent to the server. It usually indicates the request has been tampered with.
- **RhinoAccountsInvalidTokenException**
  - Exception thrown when the token returned by Rhino Accounts is not valid.
- **RhinoAccountsManager**
  - Performs various Rhino Accounts-related tasks.
- **RhinoAccountsOperationInProgressException**
  - Exception thrown when there is already a Rhino Accounts operation taking place.
- **RhinoAccountsProxyException**
  - Exception thrown when there is a problem with a proxy setting during a Rhino Accounts operation.
- **RhinoAccountsServerException**
  - Exception thrown when the Rhino Accounts server returned an unsuccessful HTTP response with a code of 400 or greater.
- **RhinoAccountsServerNotReachableException**
  - Exception thrown when the Rhino Accounts server cannot be reached due to a network problem.
- **SecretKey**
  - An instance of this of this class is given to the function you pass to ExceuteProtectedCodeAsync and             must be passed to any method of the RhinoAccountsManager that requires it. Failure to do so will throw an .
