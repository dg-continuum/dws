## Api docs

[./asyncapi.md](./asyncapi.md)

## Running

```bash
export BROADCAST_SECRET="supersecretkey"
export PORT="3000"
export HOST="0.0.0.0"
export DISCORD_PUBLIC_KEY=
export DISCORD_TOKEN=
export DISCORD_CLIENT_ID=
cargo run --release
```

## Rest

### GET `/cosmetics`

```json
{
  "cosmetics": [
    {
      "description": "White",
      "display": "WW",
      "id": 1,
      "name": "W",
      "required_flags": 1
    }
  ],
  "users": { "41a9b6aa-168a-4be8-8df8-cac17daf7384": 1 }
}
```

### POST `/broadcast`

to requires a list of uuids or nothing to send to all users.

```
authorization: $BROADCAST_SECRET
```

```json
{
  "message": "Hello world",
  "to": []
}
```

---

```
Ok
```

### GET `/metrics`

Returns a bunch of prometheus metrics

## Websockets

See the insomnia example for more detailed info,
Nonces are a optional field

### Connecting

for development none of this validated. server_id is expected to be a hashed server id that is needed for <https://wiki.vg/Protocol_Encryption#Authentication> 

```json
{
    "t": "/connect",
    "c": {
			"server_id": "Hello world from irc ws lol",
			"username": "trickedmc"
		}
}
```

---

```json
{
  "t": "/connected",
  "c": true
}
```

### Requesting user status

```json
{
  "t": "/is_online",
  "c": { "uuid": "41a9b6aa-168a-4be8-8df8-cac17daf6324", "nonce": "HI!" }
}
```

---

```json
{
  "t": "/is_online",
  "c": {
    "is_online": true,
    "uuid": "41a9b6aa-168a-4be8-8df8-cac17daf6324",
    "nonce": "HI!"
  }
}
```

### Requesting user status bulk

```json
{
  "t": "/is_online/bulk",
  "c": { "uuids": ["41a9b6aa-168a-4be8-8df8-cac17daf6324"], "nonce": "HI!" }
}
```

---

```json
{
  "t": "/is_online/bulk",
  "c": {
    "users": {
      "41a9b6aa-168a-4be8-8df8-cac17daf6324": true
    },
    "nonce": "HI!"
  }
}
```

### Pings

```json
{
  "t": "/ping"
}
```

---

```json
{
  "t": "/pong"
}
```

### Update cosmetic

```json
{
  "t": "/cosmetics/update",
  "c": {
    "cosmetic_id": 1,
    "nonce": "hi1"
  }
}
```

---

```json
{
  "t": "/cosmetics/updated",
  "c": {
    "cosmetic_id": 1,
    "nonce": "hi1"
  }
}
```

### Cosmetic Ack event

It is suggested to update cosmetics between 1-5 minutes after this event is received to account for any other updates and to not trigger ddos protection.

---

```json
{
  "t": "/cosmetics/ack"
}
```

### Irc

The irc chat can be linked to discord and you can blacklist uuids from the irc using `/irc blacklist Add <uuid>`, if a user is blacklisted irc messages send by this user will be silently ignored.

```json
{
  "t": "/irc/create",
  "c": {
    "message": "HI!"
  }
}
```

---

```json
{
  "t": "/irc/created",
  "c": {
    "message": "HI!",
    "sender": "41a9b6aa-168a-4be8-8df8-cac17daf7384",
    "date": 1668109163235
  }
}
```

### Broadcasts

Broadcasts are only received not send you can view the broadcast post request to find out how those work

---

```json
{
  "t": "/broadcast",
  "c": "Hello world"
}
```

### Errors

Errors are only recieved and look like this, errors can include a nonce for when necessary

---

```json
{
  "t": "/error",
  "c": {
    "error": "Already connected"
  }
}
```

## Cosmetics

A cosmetics file looks something like this

```json
{
  "cosmetics": [
    {
      "id": 1,
      "name": "W",
      "display": "WW",
      "description": "White",
      "required_flags": 1
    }
  ],
  "users": {
    "41a9b6aa-168a-4be8-8df8-cac17daf7384": {
      "flags": 1,
      "enabled_prefix": null
    }
  }
}
```