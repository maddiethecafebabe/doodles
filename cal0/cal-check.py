import sys

inc_vecs = {
    "ssl_cert" : {
        "offset" : 0x0AE0,
        "length" : 0x800
    },
    "private key" : {
        "offset" : 0x3AE0,
        "length" : 0x130
    },
    "deviceID" : {
        "offset" : 0x35E1,
        "length" : 0x006
    },
    "deviceiD" : {
        "offset" : 0x36E1,
        "length" : 0x006
    },
    "device_cert_ecc_p256" : {
        "offset" : 0x02B0,
        "length" : 0x180
    },
    "device_cert_rsa" : {
        "offset" : 0x3D70,
        "length" : 0x240
    },
    "device_key" : {
        "offset" : 0x3FC0,
        "length" : 0x240
    }
}

def read_at(file, offset=0x00, length=0x0F, mode="rb"):
    with open(file, mode) as fp:
        fp.seek(offset)
        return fp.read(length)

def is_blank(data : bytes):
    try:
        int(data.hex())
        return True
    except:
        return False

def check_for_incognito(cal):
    # This is disgusting, i know
    print("[*] {0:25}| Offset  | Is blanked out".format("Sectionname"))
    print("-"*55)
    for vec in inc_vecs.keys():
        print("[+] {0:25}| {1:8}| {2}".format(vec,
                                        hex(inc_vecs[vec]["offset"]),
                                        is_blank(
                                            read_at(cal,
                                                    inc_vecs[vec]["offset"],
                                                    inc_vecs[vec]["length"]))))
    print("\n[+] {0:25}| {1:8}| {2}XXXX".format("serial",
                                            "0x250",
                                            read_at(cal, 0x250, 16).decode()[:10]))
def check_encrypted(cal):
    magic = read_at(cal, 0x00, 0x04)
    if magic != b"CAL0":
        print("The PRODINFO seems to be encrypted.")
        print("Please decrypt it first!\t[Magic: {}]".format(magic))
        sys.exit(1)

def main(argv):
    if len(argv) < 2:
        print("usage: {} path/to/prodinfo".format(argv[0]))
        sys.exit(1)
    cal = argv[1]

    check_encrypted(cal)
    check_for_incognito(cal)

if __name__ == "__main__":
    main(sys.argv)
