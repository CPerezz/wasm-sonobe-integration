A prototype that uses a custom SnarkJS version to do witgen for Sonobe's folding steps without needing to do IO (creating witness.json file) 

This also aims to be a small example on how to use `Sonobe` within the browser in order to fold.

## Try it
Run `npm install`.

After that, run: `sh compile_circuits.sh && sh setup.sh`.

Once done, run `PORT=[WHATEVER_PORT_YOU_WANT] node app.js` and you just need to click the button and you'll get your witness without any IO being done and this witness will then be processed to perform a single fold with `Nova`.