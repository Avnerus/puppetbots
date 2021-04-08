import { html, render, define } from 'hybrids';
import store, {connect} from '../common/state'
import { PuppetAvatar } from './puppet-avatar'

// Beacause of using webpack exports-loader
// @ts-ignore
import { Janus } from 'janus-gateway';

const metadataLoaded = (host, e) => {
    console.log("Audio metadata loaded!");
}

const TheaterVoice =  {
    audioStream: connect(store, (state) => state.audioStream),
    identity: connect(store, (state) => state.identity),
    roomId: 0,
    streamURL: {
        set: (host, value, lastValue) => {
          console.log("Init stream", value);
          Janus.init({debug: "none", callback: () => {
              if(!Janus.isWebrtcSupported()) {
                  throw "WebRTC Not supported!";
              }
              host.janus = new Janus(
                  {
                      server: value,
                      iceServers: [
                        {url: "stun:stun.l.google.com:19302"}
                        //{url: "turn:tdialogos.aalto.fi:3478", credential: "telepuppets", username: "legroup"}
                      ],
                      success: () => {
                          console.log("Janus Init success! Attaching to streaming plugin");
                          const userId = Janus.randomString(12);

                          host.janus.attach(
                              {
                                plugin: "janus.plugin.audiobridge",
                                opaqueId: "audiobridgeclient-" + userId,
                                success: function(pluginHandle) {
                                    console.log("Attached succesfully! Joining room " + host.roomId)
                                    host.janusPlugin = pluginHandle
                                    const register = { request: "join", room: host.roomId, display: userId };
                                    host.janusPlugin.send({ message: register});
                                },
                                error: function(error) {
                                   console.warn("Error attaching")
                                   console.log(error)
                                },
                                iceState: function(state) {
                                  console.log("ICE state changed to " + state);
                                },
                                mediaState: function(medium, on) {
                                  console.log("Janus " + (on ? "started" : "stopped") + " receiving our " + medium);
                                },
                                webrtcState: function(on) {
                                  console.log("Janus says our WebRTC PeerConnection is " + (on ? "up" : "down") + " now");
                                },
                                onmessage: function(msg, jsep) {
                                  const event = msg["audiobridge"];
                                  if (event) {
                                    console.log("Event", event)
                                    if (event === 'joined') {
                                      if (msg["id"]) {
                                        console.log("Successfully joined room " + msg["room"] + " with ID " + msg["id"]);
                                        if (host.identity) {
                                          console.log("Creating offer", host.identity)
                                          host.janusPlugin.createOffer(
                                          {
                                            media: { video: false},	// This is an audio only room
                                            success: function(jsep) {
                                              console.log("Got SDP!", jsep)
                                              const publish = { request: "configure", muted: false };
                                              host.janusPlugin.send({ message: publish, jsep: jsep });
                                            },
                                            error: function(error) {
                                              console.warn("WebRTC error:", error);
                                            }
                                          });
                                        }
                                      }
                                    }
                                  }
                                  if(jsep) {
                                    console.log("Handling SDP as well...", jsep);
                                    host.janusPlugin.handleRemoteJsep({ jsep: jsep });
                                  }
                                },
                                onremotestream: function(stream) {
                                    console.log("Stream available!", stream)
                                }
                              }
                          )
                      },
                      error: (error) => {
                          //Janus.error(error);
                          console.log("Janus Error!", error);
                          throw "Janus Error " + error;
                      },
                      destroyed: () => {
                          console.log("Janus Destroyed!");
                      }
                  });
          }})
          return value;
        }
    },
    render: render(({ audioStream }) => 
        html`
          <audio 
              srcObject=${audioStream} 
              id="voice"
              onloadedmetadata=${metadataLoaded}
              autoplay
          ></video>
     `)
}

export { TheaterVoice }
