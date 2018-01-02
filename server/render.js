import React from 'react';
import { Suite } from 'benchmark';
import {renderToString as rustRenderToString } from '../native';
import {renderToString as jsRenderToString} from 'react-dom/server';

import App from '../src/components/App';

let assets;
if (process.env.NODE_ENV === 'development') {
  // Use the bundle from create-react-app's server in development mode.
  assets = {
    'main.js': '/static/js/bundle.js',
    'main.css': '',
  };
} else {
  assets = require('../build/asset-manifest.json');
}

export default function render() {
  const app = <App assets={assets} />;

  var html = jsRenderToString(app);
  var html = rustRenderToString(app);

  var suite = new Suite();
  suite
    .add('js.render', function() {
      jsRenderToString(app);
    })
    .add('rust.render', function() {
      rustRenderToString(app)
    })
    .on('cycle', function(event) {
      console.log(String(event.target));
    })
    .run({ 'async': true });

  // There's no way to render a doctype in React so prepend manually.
  // Also append a bootstrap script tag.
  return '<!DOCTYPE html>' + html;
}
