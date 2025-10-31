pub enum AuthAPIError {
	UserAlreadyExists,
	InvalidCredentials,
	UnexpectedError,
	IncorrectPassword,
	TokenCreationError,
	MissingToken,
	InvalidToken,
}
