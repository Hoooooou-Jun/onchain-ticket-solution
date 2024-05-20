export const unixTimestampConverter = (data) => {
	let timestamp = new Date(Number(data) * 1000)
	return timestamp.getUTCFullYear() + '-' +
	('0' + (timestamp.getUTCMonth() + 1)).slice(-2) + '-' +
	('0' + timestamp.getUTCDate()).slice(-2) + ' ' +
	('0' + timestamp.getUTCHours()).slice(-2) + ':' +
	('0' + timestamp.getUTCMinutes()).slice(-2);
}