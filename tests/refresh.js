const anchor = require('@coral-xyz/anchor');
const { assert } = require('chai');

describe('refresh', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.getProvider();

  it('Is initialized!', async () => {
    // Add your test here.
    const program = anchor.workspace.Refresh;
    const tx = await program.methods.initialize().rpc();
    console.log('Your transaction signature', tx);
  });

  it('Create message', async () => {
    const program = anchor.workspace.Refresh;
    let message = anchor.web3.Keypair.generate();

    const tx = await program.methods
      .createMessage('Hiii')
      .accounts({
        message: message.publicKey,
        user: provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers(message)
      .rpc();

    console.log('Your transaction signature', tx);
  });

  it('Read message', async () => {
    const program = anchor.workspace.Refresh;
    const accounts = await program.account.message.all();

    console.log(accounts);

    assert.ok(accounts.length === 1);
  });

  it('Update message', async () => {
    const program = anchor.workspace.Refresh;
    const accounts = await program.account.message.all();

    const tx = await program.methods
      .updateMessage('Updated')
      .accounts({
        message: accounts[0].publicKey,
        user: provider.publicKey,
      })
      .rpc();

    const updated = await program.account.message.all();

    assert.ok(updated[0].account.message === 'Updated');
  });
});
