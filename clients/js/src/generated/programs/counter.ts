/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Address } from '@solana/addresses';
import { getU8Encoder } from '@solana/codecs-numbers';
import { Program, ProgramWithErrors } from '@solana/programs';
import {
  CounterProgramError,
  CounterProgramErrorCode,
  getCounterProgramErrorFromCode,
} from '../errors';
import {
  ParsedCreateInstruction,
  ParsedIncrementInstruction,
} from '../instructions';
import { memcmp } from '../shared';
import { Key, getKeyEncoder } from '../types';

export const COUNTER_PROGRAM_ADDRESS =
  '95waYzr7G2pZkfjfucAYZP7JRUNzhN1EdeSXE2oZogRs' as Address<'95waYzr7G2pZkfjfucAYZP7JRUNzhN1EdeSXE2oZogRs'>;

export type CounterProgram =
  Program<'95waYzr7G2pZkfjfucAYZP7JRUNzhN1EdeSXE2oZogRs'> &
    ProgramWithErrors<CounterProgramErrorCode, CounterProgramError>;

export function getCounterProgram(): CounterProgram {
  return {
    name: 'counter',
    address: COUNTER_PROGRAM_ADDRESS,
    getErrorFromCode(code: CounterProgramErrorCode, cause?: Error) {
      return getCounterProgramErrorFromCode(code, cause);
    },
  };
}

export enum CounterAccount {
  Counter,
}

export function identifyCounterAccount(
  account: { data: Uint8Array } | Uint8Array
): CounterAccount {
  const data = account instanceof Uint8Array ? account : account.data;
  if (memcmp(data, getKeyEncoder().encode(Key.Counter), 0)) {
    return CounterAccount.Counter;
  }
  throw new Error(
    'The provided account could not be identified as a counter account.'
  );
}

export enum CounterInstruction {
  Create,
  Increment,
}

export function identifyCounterInstruction(
  instruction: { data: Uint8Array } | Uint8Array
): CounterInstruction {
  const data =
    instruction instanceof Uint8Array ? instruction : instruction.data;
  if (memcmp(data, getU8Encoder().encode(0), 0)) {
    return CounterInstruction.Create;
  }
  if (memcmp(data, getU8Encoder().encode(1), 0)) {
    return CounterInstruction.Increment;
  }
  throw new Error(
    'The provided instruction could not be identified as a counter instruction.'
  );
}

export type ParsedCounterInstruction<
  TProgram extends string = '95waYzr7G2pZkfjfucAYZP7JRUNzhN1EdeSXE2oZogRs'
> =
  | ({
      instructionType: CounterInstruction.Create;
    } & ParsedCreateInstruction<TProgram>)
  | ({
      instructionType: CounterInstruction.Increment;
    } & ParsedIncrementInstruction<TProgram>);
