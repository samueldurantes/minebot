// Cron job to hit endpoint every 14 sec to keep backend alive always
const cron = require('cron');
const https = require('https');

const config = require('./config');

const job = new cron.CronJob('*/10 * * * * *', function () {
  console.log(`Restarting server`);

  // Perform an HTTPS GET request to hit any backend api.
  https
    .get(`${config.BACKEND_URL}/health`, (res) => {
      if (res.statusCode === 200) {
        console.log('Server restarted');
      } else {
        console.error(
          `failed to restart server with status code: ${res.statusCode}`
        );
      }
    })
    .on('error', (err) => {
      console.error('Error during Restart:', err.message);
    });
});

// Export the cron job.
module.exports = {
  job,
};
