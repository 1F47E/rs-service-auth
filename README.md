```
Authentication service using JWT tokens written in Rust
```

## live demo https://react-auth-client.vercel.app/

#### envs used
- ROCKET_DATABASES - see Rocket.toml for example
- JWT_KEY - base64 encoded HS256 key

#### TODO
- [x] JWT
- [x] POST /auth/signin
- [x] POST /auth/signup
- [x] GET /token/verify
- [x] key from env
- [x] postgres as a storage
- [ ] refresh token logic
- [ ] POST /token/refresh
- [ ] GET /me
- [ ] add logging
- [ ] error user already exist not true if db error



