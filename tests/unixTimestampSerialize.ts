export const unixTimestampSerialize = (date) => {
  const eventDateBigInt = BigInt(date); // BigInt로 변환 (음수 처리를 위해)
  const buffer = Buffer.alloc(8); // Buffer를 생성하고 8바이트의 공간을 할당 (i64는 8바이트 크기)
  buffer.writeBigInt64LE(eventDateBigInt, 0); // Buffer에 BigInt 값을 little-endian 형식으로 쓰기
  return buffer;
}