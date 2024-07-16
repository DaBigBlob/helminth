# protocol v1

## overview of handshake
> reqester <--> responder
hello ->
<- hello_challenge



## overview of payload
```json
{
    version: var_int,
    ident: {
        len: var_int,
        requester_index: var_int,
        requester_priv_key_soln_to_challenge<responder_pub_key_entrypted>: [bytes + {body_ecryption_bool_byte}]
    },
    body: {
        number_of_segments: var_int,
        segments: [
            {
                destination: {
                    len: var_int,
                    idxs: <responder_pub_key_entrypted>[var_int]
                },
                data: {
                    len: var_int,
                    <responder_pub_key_entrypted>[bytes]
                }
            },
            .
            .
            .
        ]
    }
    integrity: {
        type: var_int,
        len: var_int,
        [bytes]
    }
}
```

## minimum payload
```txt
version
requester_index
requester_priv_key_soln_to_challenge.len
requester_priv_key_soln_to_challenge.{bytes} (3)
number_of_segments
destination.len
destination.idxs (3)
integrity.type
```
TOTAL: 12 bytes

### var_int
is either None, u8, u16, u32, u64, or u128  
represented by first 3 bits
following are T values:
- 000 -> None i.e. does not exist. Rest of the byte is ignored.
- 001 -> u8 -> 2^(8-3) = 2^5 = 32 states
- 010 -> u16 -> 2^(16-3) = 2^13 = 8,192 states
- 011 -> u32 -> 2^(32-3) = 2^29 = 536,870,912 states
- 100 -> u64 -> 2^(64-3) = 2^61 = 2,305,843,009,213,693,952 states
- 101 -> u128 -> 2^(128-3) = 2^125 = 4.2535295865×10^37 states
- 110 -> this value is undefined in v1
- 111 -> this value is undefined in v1

## version
- var_int -> self

## ident
// TODO

## dest
- var_int -> num_of_dests
- var_int -> dest0
- var_int -> dest1
.
.
.

## data
- var_int -> bytes_of_data
- byte
- byte
.
.
.

## intgri
- var_int -> intrigrity_type
- var_int -> num_of_intrigrity_bytes
- byte
- byte
.
.
.