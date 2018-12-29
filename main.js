const FEATURES = [
  {
    title: '`impl Trait` in return position',
    rfc: '1522-conservative-impl-trait',
    tracking: '34511',
    stabilized: {
      version: '1.26',
      pr: '49255',
    },
  },
  {
    title: '`async` as a keyword in 2018 edition',
    stabilized: {
      version: '1.28',
      pr: '50307',
    },
  },
  {
    title: '2018 edition',
    stabilized: {
      version: '1.31',
      pr: '54057',
    },
  },
  {
    title: '`Pin` as a method receiver',
    rfc: '2362',
    tracking: '55786',
    stabilized: {
      version: '1.33',
      pr: '56805',
    },
  },
  {
    title: 'Pin APIs',
    rfc: '2349-pin',
    tracking: '49150',
    stabilized: {
      version: '1.33',
      pr: '56939',
    },
  },
  {
    title: '`std::task` and `std::future`',
    rfc: '2592',
  },
  {
    title: '`async`/`await` notation',
    rfc: '2394-async_await',
    tracking: '50547',
  },
  {
    title: 'better syntax for `await` expression',
    unresolved: '2394-async_await#final-syntax-for-the-await-expression',
  },
  {
    title: 'async iterators or stream',
    unresolved: '2394-async_await#generators-and-streams',
  },
];

// The following code is modified from forge.rust-lang.org
const epochDate = new Date('2015-12-11');
const epochRelease = 5;
const releaseDuration = 7 * 6 * 86400 * 1000;
const today = new Date();
const releases = (today - epochDate) / releaseDuration | 0;
const stableMinorVersion = releases + epochRelease;
const betaMinorVersion = releases + epochRelease + 1;

const $features = document.getElementById('features');
for (const { title, rfc, tracking, stabilized, unresolved } of FEATURES) {
  const $li = $c('li');
  $features.insertBefore($li, $features.firstChild);
  // Title
  $li.innerHTML = title.replace(
    /`(.+?)`/g,
    (match, p1) => `<code>${p1}</code>`,
  );
  const appendText = text => $li.appendChild(document.createTextNode(text))
  appendText(' ');
  if (unresolved) {
    const $unresolved = rfcLink(unresolved, 'unresolved');
    $unresolved.classList.add('unresolved');
    $li.appendChild($unresolved);
    continue;
  }
  // Stablization information
  if (!stabilized) {
    $li.appendChild($c('span', {
      className: 'not-stabilized',
      textContent: 'not stabilized yet',
    }));
  } else {
    const { version, pr } = stabilized;
    $li.appendChild($c('a', {
      className: 'stabilized',
      textContent: `stabilized in ${version}`,
      href: `https://github.com/rust-lang/rust/pull/${pr}`,
      target: '_blank',
    }));
    appendText(' ');
    const [_, minor] = version.split('.').map(n => parseInt(n, 10));
    if (minor <= stableMinorVersion) {
      $li.appendChild($c('span', {
        className: 'stable',
        textContent: '[in stable]',
      }));
    } else if (minor == betaMinorVersion) {
      $li.appendChild($c('span', {
        className: 'beta',
        textContent: '[in beta]',
      }));
    } else {
      $li.appendChild($c('span', {
        className: 'nightly',
        textContent: '[in nightly]',
      }));
    }
  }
  if (rfc || tracking) {
    appendText(' / ');
  }
  // RFC link
  if (rfc) {
    $li.appendChild(rfcLink(rfc));
  }
  // Tracking issue link
  if (tracking) {
    if (rfc) {
      appendText(' / ');
    }
    $li.appendChild($c('a', {
      className: 'tracking',
      href: `https://github.com/rust-lang/rust/issues/${tracking}`,
      textContent: `#${tracking}`,
      title: 'Tracking issue',
      target: '_blank',
    }));
  }
}

function $c(tag, props = {}) {
  const elem = document.createElement(tag);
  for (const prop in props) {
    elem[prop] = props[prop];
  }
  return elem;
}

function rfcLink(rfc, text = null) {
  const $rfc = $c('a', {
    className: 'rfc',
    target: '_blank',
  });
  const dash = rfc.indexOf('-');
  let rfcId;
  if (dash === -1) {
    $rfc.href = `https://github.com/rust-lang/rfcs/pull/${rfc}`;
    rfcId = rfc;
  } else {
    const [page, frag] = rfc.split('#');
    $rfc.href = `https://rust-lang.github.io/rfcs/${page}.html`;
    if (frag) {
      $rfc.href += `#${frag}`;
    }
    rfcId = rfc.slice(0, dash);
    $rfc.classList.add('merged');
  }
  $rfc.textContent = text ? text : `RFC ${rfcId}`;
  return $rfc;
}
