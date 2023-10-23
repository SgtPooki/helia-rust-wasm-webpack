import init, {bytes_to_string} from "../pkg/index.js"
import { createHelia } from 'helia'

try {
  console.log('bytes_to_string', bytes_to_string)
} catch(e) {
  console.error("Error importing `index.js`:", e)
}

const helia = await createHelia()
window.helia = helia
console.log(`helia: `, helia);
const peers = []
helia.libp2p.addEventListener("peer:discovery", (evt) => {
  console.log(`Discovered peer ${evt.detail.id.toString()}`)
  console.log('evt.detail', evt.detail)
  testPeerMultiaddrStrings(evt.detail)
}, {once: true});

function testPeerMultiaddrStrings(peer) {
  const multiaddrs = peer.multiaddrs
  const jsMultiaddrStrings = multiaddrs.map((multiaddr) => multiaddr.toString())
  console.log(`jsMultiaddrStrings: `, jsMultiaddrStrings);
  const rustMultiaddrStrings = multiaddrs.map((ma) => {
    try {
      console.log('multiaddr: ', ma)
      /**
       * @type {Uint8Array}
       */
      const bytes = ma.bytes
      console.log('trying to call wasm bytes_to_string function with ma.bytes', bytes)
      console.log('the size of the multiaddr bytes is', bytes.length)
      const result = bytes_to_string(bytes, bytes.length)
      console.log('success:', result)
    } catch (err) {
      console.error('error:', err)
      throw err
    }
  })
  console.log(`rustMultiaddrStrings: `, rustMultiaddrStrings);
}

// helia.libp2p.addEventListener("peer:connect", (evt) => {
//   console.log(`Connected to ${evt.detail.toString()}`)
//   console.log('evt.detail', evt.detail)
//   // setMultiaddrs(helia.libp2p.getMultiaddrs())
// });

// helia.libp2p.addEventListener("peer:disconnect", (evt) => {
//   console.log(`Disconnected from ${evt.detail.toString()}`)
//   // setConnectedPeers((prev) => prev.filter((peer) => peer.toString() !== evt.detail.toString()))
//   console.log('evt.detail', evt.detail)
// });
