// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

import nacl from "tweetnacl";
import { AccountAddress } from "./account_address";

/**
 * Class for creating and managing account on Aptos network
 * 
 * Use this class to create accounts, sign transactions, and more.
 */
export class Account {
  /**
   * signing key of the account, which holds the public and private key
   */
  private readonly signingKey: nacl.SignKeyPair;

  /**
   * Account address associated with the account
   */
  private readonly accountAddress: AccountAddress;

  
  /**
   * Default constructor for Account
   * This will create a new account with a random public and private key
   */
  constructor() {
    this.signingKey = nacl.sign.keyPair();
    this.accountAddress = AccountAddress()
  }
}
