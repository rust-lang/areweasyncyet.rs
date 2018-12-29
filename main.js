const FEATURES = [
  {
    title: '`async` as a keyword in 2018 edition',
    stabilized: {
      version: '1.28',
      pr: '50307'
    }
  },
  {
    title: '2018 edition',
    stabilized: {
      version: '1.31',
      pr: '54057'
    }
  },
  {
    title: '`Pin` as a method receiver',
    rfc: '2362',
    tracking: '55786',
    stabilized: {
      version: '1.33',
      pr: '56805'
    }
  },
  {
    title: 'Pin APIs',
    rfc: '2349-pin',
    tracking: '49150',
    stabilized: {
      version: '1.33',
      pr: '56939'
    }
  },
  {
    title: '`std::task` and `std::future`',
    rfc: '2592'
  },
  {
    title: '`async`/`await` notation',
    rfc: '2394-async_await',
    tracking: '50547'
  }
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
for (const { title, rfc, tracking, stabilized } of FEATURES) {
  const $li = $c('li');
  // Title
  $li.innerHTML = title.replace(
    /`(.+?)`/g,
    (match, p1) => `<code>${p1}</code>`,
  );
  const appendText = text => $li.appendChild(document.createTextNode(text))
  // Stablization information
  appendText(' ');
  if (!stabilized) {
    $li.appendChild($c('span', {
      className: 'not-stabilized',
      textContent: 'not stabilized yet'
    }));
  } else {
    const { version, pr } = stabilized;
    $li.appendChild($c('a', {
      className: 'stabilized',
      textContent: `stabilized in ${version}`,
      href: `https://github.com/rust-lang/rust/pull/${pr}`,
      target: '_blank'
    }));
    appendText(' ');
    const [_, minor] = version.split('.').map(n => parseInt(n, 10));
    if (minor <= stableMinorVersion) {
      $li.appendChild($c('span', {
        className: 'stable',
        textContent: '[in stable]'
      }));
    } else if (minor == betaMinorVersion) {
      $li.appendChild($c('span', {
        className: 'beta',
        textContent: '[in beta]'
      }));
    } else {
      $li.appendChild($c('span', {
        className: 'nightly',
        textContent: '[in nightly]'
      }));
    }
  }
  if (rfc || tracking) {
    appendText(' / ');
  }
  // RFC link
  if (rfc) {
    const $rfc = $c('a', {
      className: 'rfc',
      target: '_blank'
    });
    const dash = rfc.indexOf('-');
    if (dash === -1) {
      $rfc.href = `https://github.com/rust-lang/rfcs/pull/${rfc}`;
      $rfc.textContent = `RFC ${rfc}`;
    } else {
      $rfc.href = `https://rust-lang.github.io/rfcs/${rfc}.html`;
      $rfc.textContent = `RFC ${rfc.slice(0, dash)}`;
      $rfc.classList.add('merged');
    }
    $li.appendChild($rfc);
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
      target: '_blank'
    }));
  }
  $features.insertBefore($li, $features.firstChild);
}

function $c(tag, props = {}) {
  const elem = document.createElement(tag);
  for (const prop in props) {
    elem[prop] = props[prop];
  }
  return elem
}
