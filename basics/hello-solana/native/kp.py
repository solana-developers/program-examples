import base58
import json
import sys

#key_array = [4,182,130,247,119,117,227,207,112,73,170,126,222,197,244,99,215,107,255,202,33,43,36,17,104,111,157,246,196,192,174,95,240,23,238,206,118,215,154,238,229,96,11,37,156,123,51,223,5,231,17,117,86,136,103,14,75,95,175,132,148,54,1,46]

def print_keys_from_file(f):
  key_array = json.load(open(f, 'r'))

  secret_key = key_array[0:32]
  public_key = key_array[32:64]

  sk = base58.b58encode(bytes(secret_key))
  pk = base58.b58encode(bytes(public_key))

  print(pk)
  print(sk)

if __name__ == "__main__":
  print_keys_from_file(sys.argv[1])