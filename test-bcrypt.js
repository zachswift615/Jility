// Quick test to verify bcrypt hash matches
const bcrypt = require('bcryptjs');

const apiKey = 'jil_live_gRz7MJMFonp5iAQ5U0KPjGFNPt2GSK4x';
const hash = '$2b$12$7Mdp/JJ5IG1PgP479oubkea6aYeHdJPEBWSEJ46GnlfEL36ySrYEu';

const result = bcrypt.compareSync(apiKey, hash);
console.log('API Key:', apiKey);
console.log('Hash:', hash);
console.log('Match:', result);
