export async function send_message(msg) {
  return chrome.runtime.sendMessage(msg);
}
