/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  output: 'standalone', // For Docker deployment
  async rewrites() {
    const backendBase = process.env.BACKEND_URL || 'http://localhost:3900';
    return [
      {
        source: '/api/:path*',
        destination: `${backendBase}/api/:path*`,
      },
    ]
  },
}

module.exports = nextConfig
