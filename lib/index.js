import React from 'react';
import ReactDOM from 'react-dom';

var addon = require('../native');
console.log('[neon]:', addon.renderToString);
function renderToString(app) {
  const Type = app.type;
  const rendered = (new Type(app.props)).render();
  return addon.renderToString(rendered);
}
module.exports = {
  renderToString: addon.renderToString,
}
