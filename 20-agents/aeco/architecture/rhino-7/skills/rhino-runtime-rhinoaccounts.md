---
name: rhino-rhino-runtime-rhinoaccounts
description: This skill encodes the rhino 7.0 surface of the Rhino.Runtime.RhinoAccounts namespace — 18 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: RhinoAccoountsProgressInfo, RhinoAccountsCannotListenException, RhinoAccountsException, RhinoAccountsAuthTokenMismatchException, RhinoAccountsGroup, RhinoAccountsInvalidResponseException, RhinoAccountsInvalidTokenException, RhinoAccountsInvalidStateException, and 10 more types.
---

# Rhino.Runtime.RhinoAccounts

Auto-generated from vendor docs for rhino 7.0. 18 types in this namespace.

## IOAuth2Token (interface)

Represents an OAuth2 Token that can be used for authorization purposes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_IOAuth2Token.htm)

### Properties
- `Exp` (Nullable<DateTime>, get) — The expiration of the token. Expired tokens will be considered invalid by the Rhino Accounts server.
- `IsExpired` (Boolean, get) — true if the token is expired; false otherwise.
- `RawToken` (String, get) — The raw token that can be passed to various servers for authorization.
- `Scope` (IReadOnlyCollection<String>, get) — The scope of the token.

## IOpenIDConnectToken (interface)

This class represents an OpenIDConnect token issued from an OpenID provider. The token is immutable.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_IOpenIDConnectToken.htm)

### Properties
- `AdminGroups` (IReadOnlyDictionary<String,RhinoAccountsGroup>, get) — Returns all the groups the user is an admin of.
- `AllGroups` (IReadOnlyDictionary<String,RhinoAccountsGroup>, get) — Returns all the groups the user is a member of.
- `AtHash` (String, get) — Access Token hash value. Its value is the base64url encoding of the left-most half of the hash of the octets of the ASCII representation of the access_token value, where the hash algorithm used is the hash algorithm used in the alg Header Parameter of the ID Token's JOSE Header. For instance, if the alg is RS256, hash the access_token value with SHA-256, then take the left-most 128 bits and base64url encode them. The at_hash value is a case sensitive string.
- `Aud` (String, get) — The id of the client (the audience) this token is intended for.
- `AuthTime` (Nullable<DateTime>, get) — Time when the End-User authentication occurred
- `Emails` (IReadOnlyCollection<String>, get) — All the emails belonging to the account the token represents.
- `EmailVerified` (Nullable<Boolean>, get) — true if all the emails in the account have been verified; false otherwise.
- `Exp` (Nullable<DateTime>, get) — The date the token expires.
- `Iat` (Nullable<DateTime>, get) — The date the token was issued.
- `IsExpired` (Boolean, get) — true if the token is expired; false otherwise.
- `Iss` (String, get) — The id of the entity that issued the token.
- `IsUpdated` (Boolean, get) — true if the token has been updated; false otherwise.
- `Locale` (String, get) — The local of the user this token represents. ISO 639-1 Alpha-2 [ISO639‑1] language code and an ISO 3166-1 Alpha-2 [ISO3166‑1] country code in, separated by a dash.
- `MemberGroups` (IReadOnlyDictionary<String,RhinoAccountsGroup>, get) — Returns all the groups the user is a member of, but not an admin or an owner.
- `Name` (String, get) — The name of the user this token represents.
- `Nonce` (String, get) — String value used to associate a Client session with an ID Token, and to mitigate replay attacks
- `OwnerGroups` (IReadOnlyDictionary<String,RhinoAccountsGroup>, get) — Returns all the groups the user is an owner of.
- `Phone` (String, get) — The phone of the user this token represents.
- `Picture` (String, get) — The url of a picture/avatar/icon of the user this token represents.
- `RawToken` (String, get) — The raw OpenIDConnect token.
- `Sub` (String, get) — The unique id for the subject this token represents.
- `UpdatedAt` (Nullable<DateTime>, get) — The last time the token was updated.

## IRhinoAccountsManager (interface)

Performs various Rhino Accounts-related tasks.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_IRhinoAccountsManager.htm)

### Methods
#### `void ExecuteProtectedCode(Action<SecretKey> protectedCode)`

Any synchronous method in the IRhinoAccountsManager class must be executed within the function passed to this method, or an InvalidOperationException will be thrown.

**Remarks:** The code within the function passed must be kept as breif as possible for performance and security reasons.

**Parameters:**
- `protectedCode` (System.Action<SecretKey>) — A function returning an awaitable task that has a SecretKey passed to it. You will need to pass this secret key to any method you wish to call within IRhinoAccountsManager.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_RhinoAccounts_IRhinoAccountsManager_ExecuteProtectedCode.htm)

#### `Task ExecuteProtectedCodeAsync(Func<SecretKey, Task> protectedCode)`

Any asynchronous method in the IRhinoAccountsManager class must be executed within the function passed to this method, or an InvalidOperationException will be thrown.

**Remarks:** The code within the function passed must be kept as breif as possible for performance and security reasons.

**Parameters:**
- `protectedCode` (System.Func<SecretKey,Task>) — A function returning an awaitable task that has a SecretKey passed to it. You will need to pass this secret key to any method you wish to call within IRhinoAccountsManager.

**Returns:** `Task` — [Missing <returns> documentation for "M:Rhino.Runtime.RhinoAccounts.IRhinoAccountsManager.ExecuteProtectedCodeAsync(System.Func{Rhino.Runtime.RhinoAccounts.SecretKey,System.Threading.Tasks.Task})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_RhinoAccounts_IRhinoAccountsManager_ExecuteProtectedCodeAsync.htm)

#### `Task<Tuple<IOpenIDConnectToken, IOAuth2Token>> GetAuthTokensAsync(string clientId, string clientSecret, IEnumerable<string> scope, string prompt, Nullable<int> maxAge, bool showUI, IProgress<RhinoAccoountsProgressInfo> progress, SecretKey secretKey, CancellationToken cancellationToken)`

Asynchronously retrieves auth tokens with the given criteria from the Rhino Accounts server.

**Parameters:**
- `clientId` (System.String) — The unique id of the client registered in Rhino Accounts.
- `clientSecret` (System.String) — The secret of the client registered in Rhino Accounts
- `scope` (System.Collections.Generic.IEnumerable<String>) — The scope desired for the tokens. Valid scope values can be found in the Rhino Accounts documentation.
- `prompt` (System.String) — The prompt of the request. See Rhino Accounts documentation for details. You may pass null if no prompt is desired.
- `maxAge` (System.Nullable<Int32>) — The maxAge of the request. See Rhino Accounts documentation for details. You may pass null if no maxAge should be enforced.
- `showUI` (System.Boolean) — true if the user should see a UI showing the progress of the operation and a way to cancel it, or false if the UI should not be displayed. If false, it is strongly recommended that you pass a progress object and display your own UI to the user.
- `progress` (System.IProgress<RhinoAccoountsProgressInfo>) — An object that will report the progress of the operation to the caller. If no progress is needed, you may pass null.
- `secretKey` (Rhino.Runtime.RhinoAccounts.SecretKey) — A special key that was handed to you in ExecuteProtectedCodeAsync(Func<SecretKey, Task>)
- `cancellationToken` (System.Threading.CancellationToken) — A token that can be used to signal that the operation should be cancelled.

**Returns:** `Task<Tuple<IOpenIDConnectToken,IOAuth2Token>>` — The auth tokens requested.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_RhinoAccounts_IRhinoAccountsManager_GetAuthTokensAsync_1.htm)

#### `Task<Tuple<IOpenIDConnectToken, IOAuth2Token>> GetAuthTokensAsync(string clientId, string clientSecret, SecretKey secretKey, CancellationToken cancellationToken)`

Asynchronously retrieves auth tokens with the given criteria from the Rhino Accounts server.

**Remarks:** This method includes a default scope of 'openid', 'email', 'profile', and 'groups'. It should be suffifient for most workflows. This method shows the user a UI showing them the progress of the operation, and allows them to cancel it at any time. It also stores the retrieved tokens in a secure cache so they can be retrieved in the future by calling TryGetAuthTokens(String, SecretKey)

**Parameters:**
- `clientId` (System.String) — The unique id of the client registered in Rhino Accounts.
- `clientSecret` (System.String) — The secret of the client registered in Rhino Accounts
- `secretKey` (Rhino.Runtime.RhinoAccounts.SecretKey) — A special key that was handed to you in ExecuteProtectedCodeAsync(Func<SecretKey, Task>)
- `cancellationToken` (System.Threading.CancellationToken) — A token that can be used to signal that the operation should be cancelled.

**Returns:** `Task<Tuple<IOpenIDConnectToken,IOAuth2Token>>` — The auth tokens requested.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_RhinoAccounts_IRhinoAccountsManager_GetAuthTokensAsync.htm)

#### `Task RevokeAuthTokenAsync(IOAuth2Token oauth2Token, SecretKey secretKey, CancellationToken cancellationToken)`

Invalidates/revokes an IOAuth2Token object from the Rhino Accounts server.

**Remarks:** If the OAuth 2 token is stored in the local cache, it will be removed from the cache as well.

**Parameters:**
- `oauth2Token` (Rhino.Runtime.RhinoAccounts.IOAuth2Token) — The token to revoke.
- `secretKey` (Rhino.Runtime.RhinoAccounts.SecretKey) — A special key that was handed to you in ExecuteProtectedCodeAsync(Func<SecretKey, Task>)
- `cancellationToken` (System.Threading.CancellationToken) — A token that can be used to signal that the operation should be cancelled.

**Returns:** `Task` — [Missing <returns> documentation for "M:Rhino.Runtime.RhinoAccounts.IRhinoAccountsManager.RevokeAuthTokenAsync(Rhino.Runtime.RhinoAccounts.IOAuth2Token,Rhino.Runtime.RhinoAccounts.SecretKey,System.Threading.CancellationToken)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_RhinoAccounts_IRhinoAccountsManager_RevokeAuthTokenAsync.htm)

#### `Tuple<IOpenIDConnectToken, IOAuth2Token> TryGetAuthTokens(string clientId, IEnumerable<string> scope, SecretKey secretKey)`

Attempts to return cached auth tokens that match the given criteria if any have been stored in cache.

**Parameters:**
- `clientId` (System.String) — The unique id of the client registered in Rhino Accounts.
- `scope` (System.Collections.Generic.IEnumerable<String>) — The scope desired for the tokens. Valid scope values can be found in the Rhino Accounts documentation.
- `secretKey` (Rhino.Runtime.RhinoAccounts.SecretKey) — A special key that was handed to you in ExecuteProtectedCodeAsync(Func<SecretKey, Task>)

**Returns:** `Tuple<IOpenIDConnectToken,IOAuth2Token>` — Cached tokens matching the exact criteria passed, or null if none can be found matching the criteria.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_RhinoAccounts_IRhinoAccountsManager_TryGetAuthTokens_1.htm)

#### `Tuple<IOpenIDConnectToken, IOAuth2Token> TryGetAuthTokens(string clientId, SecretKey secretKey)`

Attempts to return cached auth tokens that match the given criteria if any have been stored in cache.

**Parameters:**
- `clientId` (System.String) — The unique id of the client registered in Rhino Accounts.
- `secretKey` (Rhino.Runtime.RhinoAccounts.SecretKey) — A special key that was handed to you in ExecuteProtectedCodeAsync(Func<SecretKey, Task>)

**Returns:** `Tuple<IOpenIDConnectToken,IOAuth2Token>` — Cached tokens matching the exact criteria passed, or null if none can be found matching the criteria.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_RhinoAccounts_IRhinoAccountsManager_TryGetAuthTokens.htm)

#### `Task<IOpenIDConnectToken> UpdateOpenIDConnectTokenAsync(IOpenIDConnectToken currentToken, IOAuth2Token oauth2Token, SecretKey secretKey, CancellationToken cancellationToken)`

Updates an OpenID Connect token so that it contains the latest user information by contacting the Rhino Account's server userinfo endpoint using a compatible O

**Remarks:** If the OpenID Connect token is currently stored in the local cache, it will be updated to reflect the new token.

**Parameters:**
- `currentToken` (Rhino.Runtime.RhinoAccounts.IOpenIDConnectToken) — The existing OpenID Connect token that you wish to updated with the latest user information.
- `oauth2Token` (Rhino.Runtime.RhinoAccounts.IOAuth2Token) — A valid OAuth2 token used for authorization. The OAuth2 token must have been issued together with the OpenID Connect token passed or a RhinoAccountsAuthTokenMismatchException will be thrown.
- `secretKey` (Rhino.Runtime.RhinoAccounts.SecretKey) — A special key that was handed to you in ExecuteProtectedCodeAsync(Func<SecretKey, Task>)
- `cancellationToken` (System.Threading.CancellationToken) — A token that can be used to signal that the operation should be cancelled.

**Returns:** `Task<IOpenIDConnectToken>` — The updated OpenIDConnectToken based on the original token passed to this method.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_RhinoAccounts_IRhinoAccountsManager_UpdateOpenIDConnectTokenAsync.htm)

## ProgressState (enum)

Describes the state of progress.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_ProgressState.htm)

### Values
- `AwaitingLogin` = `0` — The task is awaiting for the user to login.
- `RetrievingTokens` = `1` — The task is now negotiating with the remote server for auth tokens.
- `AwaitingRedirect` = `2` — The task is now waiting for redirection to occur.
- `Other` = `3` — Other

## RhinoAccoountsProgressInfo (class)

This class is designed to convey the progress of an asynchronous operation through .NET's IProgress<T> interface. It can be used by callers of such tasks to relay the tasks' progress, as well as useful metadata information that may be of interest.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_RhinoAccoountsProgressInfo.htm)

### Constructors
- `public RhinoAccoountsProgressInfo(ProgressState state, Dictionary<Object, Object> metadata = null, string customDescription = null)` — Creates a new instance.

### Properties
- `Description` (String, get) — A localized description of the state that may be shown to the user of the application.
- `Metadata` (Dictionary<Object,Object>, get) — Any metadata of interest that may be used by the caller of a task.
- `State` (ProgressState, get) — The state to report.

## RhinoAccountsAuthTokenMismatchException (class)

Exception thrown when the currently logged in user is different from the newly logged in user.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_RhinoAccountsAuthTokenMismatchException.htm)

### Constructors
- `public RhinoAccountsAuthTokenMismatchException(Exception innerException = null)` — Generates a new instance of the exception.
- `public RhinoAccountsAuthTokenMismatchException(string message, Exception innerException = null)` — Generates a new instance of the exception.
- `public RhinoAccountsAuthTokenMismatchException(string currentUsername, string newUsername, Exception innerException = null)` — Generates a new instance of the exception.

## RhinoAccountsCannotListenException (class)

Exception thrown when there is no port available on the machine for Rhino to listen for Rhino Accounts' response.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_RhinoAccountsCannotListenException.htm)

### Constructors
- `public RhinoAccountsCannotListenException(Exception innerException = null)` — Generates a new instance of the exception.
- `public RhinoAccountsCannotListenException(string message, Exception innerException = null)` — Generates a new instance of the exception.

## RhinoAccountsException (class)

Base exception for all Rhino Accounts operations.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_RhinoAccountsException.htm)

### Constructors
- `public RhinoAccountsException(Exception innerException = null)` — Generates a new instance of the exception.
- `public RhinoAccountsException(string message, Exception innerException = null)` — Generates a new instance of the exception.

## RhinoAccountsGroup (class)

Represents a Rhino Accounts group.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_RhinoAccountsGroup.htm)

### Constructors
- `public RhinoAccountsGroup(string id, string name)` — Represents a group in Rhino Accounts. Groups are a collection of individual members that can share resources.

### Properties
- `Id` (String, get) — The id of the group. The id is unique to a group within Rhino Accounts.
- `Name` (String, get) — The name of the group. The name can be changed at anytime by group members.

## RhinoAccountsInvalidResponseException (class)

Exception thrown when the response returned by Rhino Accounts is not valid.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_RhinoAccountsInvalidResponseException.htm)

### Constructors
- `public RhinoAccountsInvalidResponseException(Exception innerException = null)` — Generates a new instance of the exception.
- `public RhinoAccountsInvalidResponseException(string message, Exception innerException = null)` — Generates a new instance of the exception.

## RhinoAccountsInvalidStateException (class)

Occurs when the state returned by the Rhino Accounts server is not the same as the one sent to the server. It usually indicates the request has been tampered with.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_RhinoAccountsInvalidStateException.htm)

### Constructors
- `public RhinoAccountsInvalidStateException(Exception innerException = null)` — Generates a new instance of the exception.
- `public RhinoAccountsInvalidStateException(string message, Exception innerException = null)` — Generates a new instance of the exception.

## RhinoAccountsInvalidTokenException (class)

Exception thrown when the token returned by Rhino Accounts is not valid.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_RhinoAccountsInvalidTokenException.htm)

### Constructors
- `public RhinoAccountsInvalidTokenException(Exception innerException = null)` — Generates a new instance of the exception.
- `public RhinoAccountsInvalidTokenException(string message, Exception innerException = null)` — Generates a new instance of the exception.

## RhinoAccountsManager (class)

Performs various Rhino Accounts-related tasks.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_RhinoAccountsManager.htm)

### Methods
#### `public static void ExecuteProtectedCode(Action<SecretKey> protectedCode)`

Any synchronous method in the IRhinoAccountsManager class must be executed within the function passed to this method, or an InvalidOperationException will be thrown.

**Remarks:** The code within the function passed must be kept as breif as possible for performance and security reasons.

**Parameters:**
- `protectedCode` (System.Action<SecretKey>) — A function returning an awaitable task that has a SecretKey passed to it. You will need to pass this secret key to any method you wish to call within IRhinoAccountsManager.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_RhinoAccounts_RhinoAccountsManager_ExecuteProtectedCode.htm)

#### `public static Task ExecuteProtectedCodeAsync(Func<SecretKey, Task> protectedCode)`

Any asynchronous method in the IRhinoAccountsManager class must be executed within the function passed to this method, or an InvalidOperationException will be thrown.

**Remarks:** The code within the function passed must be kept as breif as possible for performance and security reasons.

**Parameters:**
- `protectedCode` (System.Func<SecretKey,Task>) — A function returning an awaitable task that has a SecretKey passed to it. You will need to pass this secret key to any method you wish to call within IRhinoAccountsManager.

**Returns:** `Task` — [Missing <returns> documentation for "M:Rhino.Runtime.RhinoAccounts.RhinoAccountsManager.ExecuteProtectedCodeAsync(System.Func{Rhino.Runtime.RhinoAccounts.SecretKey,System.Threading.Tasks.Task})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_RhinoAccounts_RhinoAccountsManager_ExecuteProtectedCodeAsync.htm)

#### `public static Task<Tuple<IOpenIDConnectToken, IOAuth2Token>> GetAuthTokensAsync(string clientId, string clientSecret, IEnumerable<string> scope, string prompt, Nullable<int> maxAge, bool showUI, IProgress<RhinoAccoountsProgressInfo> progress, SecretKey secretKey, CancellationToken cancellationToken)`

Asynchronously retrieves auth tokens with the given criteria from the Rhino Accounts server.

**Parameters:**
- `clientId` (System.String) — The unique id of the client registered in Rhino Accounts.
- `clientSecret` (System.String) — The secret of the client registered in Rhino Accounts
- `scope` (System.Collections.Generic.IEnumerable<String>) — The scope desired for the tokens. Valid scope values can be found in the Rhino Accounts documentation.
- `prompt` (System.String) — The prompt of the request. See Rhino Accounts documentation for details. You may pass null if no prompt is desired.
- `maxAge` (System.Nullable<Int32>) — The maxAge of the request. See Rhino Accounts documentation for details. You may pass null if no maxAge should be enforced.
- `showUI` (System.Boolean) — true if the user should see a UI showing the progress of the operation and a way to cancel it, or false if the UI should not be displayed. If false, it is strongly recommended that you pass a progress object and display your own UI to the user.
- `progress` (System.IProgress<RhinoAccoountsProgressInfo>) — An object that will report the progress of the operation to the caller. If no progress is needed, you may pass null.
- `secretKey` (Rhino.Runtime.RhinoAccounts.SecretKey) — A special key that was handed to you in ExecuteProtectedCodeAsync(Func<SecretKey, Task>)
- `cancellationToken` (System.Threading.CancellationToken) — A token that can be used to signal that the operation should be cancelled.

**Returns:** `Task<Tuple<IOpenIDConnectToken,IOAuth2Token>>` — The auth tokens requested.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_RhinoAccounts_RhinoAccountsManager_GetAuthTokensAsync_1.htm)

#### `public static Task<Tuple<IOpenIDConnectToken, IOAuth2Token>> GetAuthTokensAsync(string clientId, string clientSecret, SecretKey secretKey, CancellationToken cancellationToken)`

Asynchronously retrieves auth tokens with the given criteria from the Rhino Accounts server.

**Remarks:** This method includes a default scope of 'openid', 'email', 'profile', and 'groups'. It should be suffifient for most workflows. This method shows the user a UI showing them the progress of the operation, and allows them to cancel it at any time. It also stores the retrieved tokens in a secure cache so they can be retrieved in the future by calling TryGetAuthTokens(String, SecretKey)

**Parameters:**
- `clientId` (System.String) — The unique id of the client registered in Rhino Accounts.
- `clientSecret` (System.String) — The secret of the client registered in Rhino Accounts
- `secretKey` (Rhino.Runtime.RhinoAccounts.SecretKey) — A special key that was handed to you in ExecuteProtectedCodeAsync(Func<SecretKey, Task>)
- `cancellationToken` (System.Threading.CancellationToken) — A token that can be used to signal that the operation should be cancelled.

**Returns:** `Task<Tuple<IOpenIDConnectToken,IOAuth2Token>>` — The auth tokens requested.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_RhinoAccounts_RhinoAccountsManager_GetAuthTokensAsync.htm)

#### `public static Task RevokeAuthTokenAsync(IOAuth2Token oauth2Token, SecretKey secretKey, CancellationToken cancellationToken)`

Invalidates/revokes an IOAuth2Token object from the Rhino Accounts server.

**Remarks:** If the OAuth 2 token is stored in the local cache, it will be removed from the cache as well.

**Parameters:**
- `oauth2Token` (Rhino.Runtime.RhinoAccounts.IOAuth2Token) — The token to revoke.
- `secretKey` (Rhino.Runtime.RhinoAccounts.SecretKey) — A special key that was handed to you in ExecuteProtectedCodeAsync(Func<SecretKey, Task>)
- `cancellationToken` (System.Threading.CancellationToken) — A token that can be used to signal that the operation should be cancelled.

**Returns:** `Task` — [Missing <returns> documentation for "M:Rhino.Runtime.RhinoAccounts.RhinoAccountsManager.RevokeAuthTokenAsync(Rhino.Runtime.RhinoAccounts.IOAuth2Token,Rhino.Runtime.RhinoAccounts.SecretKey,System.Threading.CancellationToken)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_RhinoAccounts_RhinoAccountsManager_RevokeAuthTokenAsync.htm)

#### `public static Tuple<IOpenIDConnectToken, IOAuth2Token> TryGetAuthTokens(string clientId, IEnumerable<string> scope, SecretKey secretKey)`

Attempts to return cached auth tokens that match the given criteria if any have been stored in cache.

**Parameters:**
- `clientId` (System.String) — The unique id of the client registered in Rhino Accounts.
- `scope` (System.Collections.Generic.IEnumerable<String>) — The scope desired for the tokens. Valid scope values can be found in the Rhino Accounts documentation.
- `secretKey` (Rhino.Runtime.RhinoAccounts.SecretKey) — A special key that was handed to you in ExecuteProtectedCodeAsync(Func<SecretKey, Task>)

**Returns:** `Tuple<IOpenIDConnectToken,IOAuth2Token>` — Cached tokens matching the exact criteria passed, or null if none can be found matching the criteria.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_RhinoAccounts_RhinoAccountsManager_TryGetAuthTokens_1.htm)

#### `public static Tuple<IOpenIDConnectToken, IOAuth2Token> TryGetAuthTokens(string clientId, SecretKey secretKey)`

Attempts to return cached auth tokens that match the given criteria if any have been stored in cache.

**Parameters:**
- `clientId` (System.String) — The unique id of the client registered in Rhino Accounts.
- `secretKey` (Rhino.Runtime.RhinoAccounts.SecretKey) — A special key that was handed to you in ExecuteProtectedCodeAsync(Func<SecretKey, Task>)

**Returns:** `Tuple<IOpenIDConnectToken,IOAuth2Token>` — Cached tokens matching the exact criteria passed, or null if none can be found matching the criteria.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_RhinoAccounts_RhinoAccountsManager_TryGetAuthTokens.htm)

#### `public static Task<IOpenIDConnectToken> UpdateOpenIDConnectTokenAsync(IOpenIDConnectToken currentToken, IOAuth2Token oauth2Token, SecretKey secretKey, CancellationToken cancellationToken)`

Updates an OpenID Connect token so that it contains the latest user information by contacting the Rhino Account's server userinfo endpoint using a compatible O

**Remarks:** If the OpenID Connect token is currently stored in the local cache, it will be updated to reflect the new token.

**Parameters:**
- `currentToken` (Rhino.Runtime.RhinoAccounts.IOpenIDConnectToken) — The existing OpenID Connect token that you wish to updated with the latest user information.
- `oauth2Token` (Rhino.Runtime.RhinoAccounts.IOAuth2Token) — A valid OAuth2 token used for authorization. The OAuth2 token must have been issued together with the OpenID Connect token passed or a RhinoAccountsAuthTokenMismatchException will be thrown.
- `secretKey` (Rhino.Runtime.RhinoAccounts.SecretKey) — A special key that was handed to you in ExecuteProtectedCodeAsync(Func<SecretKey, Task>)
- `cancellationToken` (System.Threading.CancellationToken) — A token that can be used to signal that the operation should be cancelled.

**Returns:** `Task<IOpenIDConnectToken>` — The updated OpenIDConnectToken based on the original token passed to this method.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_RhinoAccounts_RhinoAccountsManager_UpdateOpenIDConnectTokenAsync.htm)

## RhinoAccountsOperationInProgressException (class)

Exception thrown when there is already a Rhino Accounts operation taking place.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_RhinoAccountsOperationInProgressException.htm)

### Constructors
- `public RhinoAccountsOperationInProgressException(Assembly assembly, Exception innerException = null)` — Generates a new instance of the exception.
- `public RhinoAccountsOperationInProgressException(string message, Exception innerException = null)` — Generates a new instance of the exception.

## RhinoAccountsProxyException (class)

Exception thrown when there is a problem with a proxy setting during a Rhino Accounts operation.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_RhinoAccountsProxyException.htm)

### Constructors
- `public RhinoAccountsProxyException(Exception innerException = null)` — Generates a new instance of the exception.
- `public RhinoAccountsProxyException(string message, Exception innerException = null)` — Generates a new instance of the exception.

## RhinoAccountsServerException (class)

Exception thrown when the Rhino Accounts server returned an unsuccessful HTTP response with a code of 400 or greater.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_RhinoAccountsServerException.htm)

### Constructors
- `public RhinoAccountsServerException(Exception innerException = null)` — Generates a new instance of the exception.
- `public RhinoAccountsServerException(string message, Exception innerException = null)` — Generates a new instance of the exception.

## RhinoAccountsServerNotReachableException (class)

Exception thrown when the Rhino Accounts server cannot be reached due to a network problem.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_RhinoAccountsServerNotReachableException.htm)

### Constructors
- `public RhinoAccountsServerNotReachableException(Exception innerException = null)` — Generates a new instance of the exception.
- `public RhinoAccountsServerNotReachableException(string message, Exception innerException = null)` — Generates a new instance of the exception.

## SecretKey (class)

An instance of this of this class is given to the function you pass to ExceuteProtectedCodeAsync and must be passed to any method of the RhinoAccountsManager that requires it. Failure to do so will throw an InvalidOperationException.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Runtime_RhinoAccounts_SecretKey.htm)

### Constructors
- `public SecretKey()` — Initializes a new instance of the SecretKey class

