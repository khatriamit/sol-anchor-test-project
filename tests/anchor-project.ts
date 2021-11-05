import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { AnchorProject } from '../target/types/anchor_project';

describe('anchor-project', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.AnchorProject as Program<AnchorProject>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
