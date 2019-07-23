

module.exports = (scenario) => {
  scenario('get user address', async(s, t, {liza}) => {
    const Host_Doc = {
      host_doc:{
      kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
    }}

    const verified = await liza.call("host", "register_as_host", Host_Doc);
    console.log("verified:: ",verified);
    t.equal(verified.Ok.length, 46)

    const is_verified = await liza.call("host", "is_registered_as_host", {});
    console.log("is verified?:: ",is_verified);
    t.equal(is_verified.Ok.links.length, 1)
  })
}
