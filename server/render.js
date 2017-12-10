import React from 'react';
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
  var times = 10;
  var html = jsRenderToString(<App assets={assets} />);
  console.time('jsRenderToString');
  for (var i = 0; i < times; i++) {
    var html = jsRenderToString(<App assets={assets} />);
  }
  console.timeEnd('jsRenderToString');
  console.time('rustRenderToString');
  for (var i = 0; i < times; i++) {
    var html = rustRenderToString(<App assets={assets} />);
  }
  console.timeEnd('rustRenderToString');
  // There's no way to render a doctype in React so prepend manually.
  // Also append a bootstrap script tag.
  return '<!DOCTYPE html>' + html;
}
