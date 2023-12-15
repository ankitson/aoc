
while (1) {
    u32 as_int;
    std::memcpy(&as_int, buf + buf_idx, 4);
    as_int -= 0x30303030;
    if (as_int & 0xf0f0f0f0) break;
    u32 compacted = _pext_u32(as_int, 0x0f0f0f0f);
    if (inv_digits_10k[compacted] == u16(~u16())) break;
    tmp *= 10000;
    tmp += inv_digits_10k[compacted];
    buf_idx += 4;
}
while (buf[buf_idx] > ' ') {
    tmp = tmp * 10 + buf[buf_idx] - '0';
    ++buf_idx;
}
reader.consume(buf_idx);