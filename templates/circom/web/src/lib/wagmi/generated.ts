import { createReadContract, createSimulateContract, createWriteContract } from 'wagmi/codegen'

import { createUseReadContract, createUseSimulateContract, createUseWriteContract } from 'wagmi/codegen'

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Counter
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/**
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const counterAbi = [
  {
    type: 'function',
    inputs: [],
    name: 'increment',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'number',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'newNumber', internalType: 'uint256', type: 'uint256' }],
    name: 'setNumber',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const

/**
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const counterAddress = {
  31337: '0x5FbDB2315678afecb367f032d93F642f64180aa3',
  11155111: '0xFd4364354145989210ee6d93a36B4eC7D8eC65EA',
} as const

/**
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const counterConfig = {
  address: counterAddress,
  abi: counterAbi,
} as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Groth16Verifier
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const groth16VerifierAbi = [
  {
    type: 'function',
    inputs: [
      { name: '_pA', internalType: 'uint256[2]', type: 'uint256[2]' },
      { name: '_pB', internalType: 'uint256[2][2]', type: 'uint256[2][2]' },
      { name: '_pC', internalType: 'uint256[2]', type: 'uint256[2]' },
      { name: '_pubSignals', internalType: 'uint256[32]', type: 'uint256[32]' },
    ],
    name: 'verifyProof',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IMulticall3
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const iMulticall3Abi = [
  {
    type: 'function',
    inputs: [
      {
        name: 'calls',
        internalType: 'struct IMulticall3.Call[]',
        type: 'tuple[]',
        components: [
          { name: 'target', internalType: 'address', type: 'address' },
          { name: 'callData', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    name: 'aggregate',
    outputs: [
      { name: 'blockNumber', internalType: 'uint256', type: 'uint256' },
      { name: 'returnData', internalType: 'bytes[]', type: 'bytes[]' },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'calls',
        internalType: 'struct IMulticall3.Call3[]',
        type: 'tuple[]',
        components: [
          { name: 'target', internalType: 'address', type: 'address' },
          { name: 'allowFailure', internalType: 'bool', type: 'bool' },
          { name: 'callData', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    name: 'aggregate3',
    outputs: [
      {
        name: 'returnData',
        internalType: 'struct IMulticall3.Result[]',
        type: 'tuple[]',
        components: [
          { name: 'success', internalType: 'bool', type: 'bool' },
          { name: 'returnData', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'calls',
        internalType: 'struct IMulticall3.Call3Value[]',
        type: 'tuple[]',
        components: [
          { name: 'target', internalType: 'address', type: 'address' },
          { name: 'allowFailure', internalType: 'bool', type: 'bool' },
          { name: 'value', internalType: 'uint256', type: 'uint256' },
          { name: 'callData', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    name: 'aggregate3Value',
    outputs: [
      {
        name: 'returnData',
        internalType: 'struct IMulticall3.Result[]',
        type: 'tuple[]',
        components: [
          { name: 'success', internalType: 'bool', type: 'bool' },
          { name: 'returnData', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'calls',
        internalType: 'struct IMulticall3.Call[]',
        type: 'tuple[]',
        components: [
          { name: 'target', internalType: 'address', type: 'address' },
          { name: 'callData', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    name: 'blockAndAggregate',
    outputs: [
      { name: 'blockNumber', internalType: 'uint256', type: 'uint256' },
      { name: 'blockHash', internalType: 'bytes32', type: 'bytes32' },
      {
        name: 'returnData',
        internalType: 'struct IMulticall3.Result[]',
        type: 'tuple[]',
        components: [
          { name: 'success', internalType: 'bool', type: 'bool' },
          { name: 'returnData', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getBasefee',
    outputs: [{ name: 'basefee', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'blockNumber', internalType: 'uint256', type: 'uint256' }],
    name: 'getBlockHash',
    outputs: [{ name: 'blockHash', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getBlockNumber',
    outputs: [
      { name: 'blockNumber', internalType: 'uint256', type: 'uint256' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getChainId',
    outputs: [{ name: 'chainid', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getCurrentBlockCoinbase',
    outputs: [{ name: 'coinbase', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getCurrentBlockDifficulty',
    outputs: [{ name: 'difficulty', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getCurrentBlockGasLimit',
    outputs: [{ name: 'gaslimit', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getCurrentBlockTimestamp',
    outputs: [{ name: 'timestamp', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'addr', internalType: 'address', type: 'address' }],
    name: 'getEthBalance',
    outputs: [{ name: 'balance', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getLastBlockHash',
    outputs: [{ name: 'blockHash', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'requireSuccess', internalType: 'bool', type: 'bool' },
      {
        name: 'calls',
        internalType: 'struct IMulticall3.Call[]',
        type: 'tuple[]',
        components: [
          { name: 'target', internalType: 'address', type: 'address' },
          { name: 'callData', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    name: 'tryAggregate',
    outputs: [
      {
        name: 'returnData',
        internalType: 'struct IMulticall3.Result[]',
        type: 'tuple[]',
        components: [
          { name: 'success', internalType: 'bool', type: 'bool' },
          { name: 'returnData', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'requireSuccess', internalType: 'bool', type: 'bool' },
      {
        name: 'calls',
        internalType: 'struct IMulticall3.Call[]',
        type: 'tuple[]',
        components: [
          { name: 'target', internalType: 'address', type: 'address' },
          { name: 'callData', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    name: 'tryBlockAndAggregate',
    outputs: [
      { name: 'blockNumber', internalType: 'uint256', type: 'uint256' },
      { name: 'blockHash', internalType: 'bytes32', type: 'bytes32' },
      {
        name: 'returnData',
        internalType: 'struct IMulticall3.Result[]',
        type: 'tuple[]',
        components: [
          { name: 'success', internalType: 'bool', type: 'bool' },
          { name: 'returnData', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    stateMutability: 'payable',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Action
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/**
 * Wraps __{@link readContract}__ with `abi` set to __{@link counterAbi}__
 *
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const readCounter = /*#__PURE__*/ createReadContract({
  abi: counterAbi,
  address: counterAddress,
})

/**
 * Wraps __{@link readContract}__ with `abi` set to __{@link counterAbi}__ and `functionName` set to `"number"`
 *
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const readCounterNumber = /*#__PURE__*/ createReadContract({
  abi: counterAbi,
  address: counterAddress,
  functionName: 'number',
})

/**
 * Wraps __{@link writeContract}__ with `abi` set to __{@link counterAbi}__
 *
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const writeCounter = /*#__PURE__*/ createWriteContract({
  abi: counterAbi,
  address: counterAddress,
})

/**
 * Wraps __{@link writeContract}__ with `abi` set to __{@link counterAbi}__ and `functionName` set to `"increment"`
 *
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const writeCounterIncrement = /*#__PURE__*/ createWriteContract({
  abi: counterAbi,
  address: counterAddress,
  functionName: 'increment',
})

/**
 * Wraps __{@link writeContract}__ with `abi` set to __{@link counterAbi}__ and `functionName` set to `"setNumber"`
 *
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const writeCounterSetNumber = /*#__PURE__*/ createWriteContract({
  abi: counterAbi,
  address: counterAddress,
  functionName: 'setNumber',
})

/**
 * Wraps __{@link simulateContract}__ with `abi` set to __{@link counterAbi}__
 *
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const simulateCounter = /*#__PURE__*/ createSimulateContract({
  abi: counterAbi,
  address: counterAddress,
})

/**
 * Wraps __{@link simulateContract}__ with `abi` set to __{@link counterAbi}__ and `functionName` set to `"increment"`
 *
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const simulateCounterIncrement = /*#__PURE__*/ createSimulateContract({
  abi: counterAbi,
  address: counterAddress,
  functionName: 'increment',
})

/**
 * Wraps __{@link simulateContract}__ with `abi` set to __{@link counterAbi}__ and `functionName` set to `"setNumber"`
 *
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const simulateCounterSetNumber = /*#__PURE__*/ createSimulateContract({
  abi: counterAbi,
  address: counterAddress,
  functionName: 'setNumber',
})

/**
 * Wraps __{@link readContract}__ with `abi` set to __{@link groth16VerifierAbi}__
 */
export const readGroth16Verifier = /*#__PURE__*/ createReadContract({
  abi: groth16VerifierAbi,
})

/**
 * Wraps __{@link readContract}__ with `abi` set to __{@link groth16VerifierAbi}__ and `functionName` set to `"verifyProof"`
 */
export const readGroth16VerifierVerifyProof = /*#__PURE__*/ createReadContract({
  abi: groth16VerifierAbi,
  functionName: 'verifyProof',
})

/**
 * Wraps __{@link readContract}__ with `abi` set to __{@link iMulticall3Abi}__
 */
export const readIMulticall3 = /*#__PURE__*/ createReadContract({
  abi: iMulticall3Abi,
})

/**
 * Wraps __{@link readContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getBasefee"`
 */
export const readIMulticall3GetBasefee = /*#__PURE__*/ createReadContract({
  abi: iMulticall3Abi,
  functionName: 'getBasefee',
})

/**
 * Wraps __{@link readContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getBlockHash"`
 */
export const readIMulticall3GetBlockHash = /*#__PURE__*/ createReadContract({
  abi: iMulticall3Abi,
  functionName: 'getBlockHash',
})

/**
 * Wraps __{@link readContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getBlockNumber"`
 */
export const readIMulticall3GetBlockNumber = /*#__PURE__*/ createReadContract({
  abi: iMulticall3Abi,
  functionName: 'getBlockNumber',
})

/**
 * Wraps __{@link readContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getChainId"`
 */
export const readIMulticall3GetChainId = /*#__PURE__*/ createReadContract({
  abi: iMulticall3Abi,
  functionName: 'getChainId',
})

/**
 * Wraps __{@link readContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getCurrentBlockCoinbase"`
 */
export const readIMulticall3GetCurrentBlockCoinbase = /*#__PURE__*/ createReadContract({
  abi: iMulticall3Abi,
  functionName: 'getCurrentBlockCoinbase',
})

/**
 * Wraps __{@link readContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getCurrentBlockDifficulty"`
 */
export const readIMulticall3GetCurrentBlockDifficulty = /*#__PURE__*/ createReadContract({
  abi: iMulticall3Abi,
  functionName: 'getCurrentBlockDifficulty',
})

/**
 * Wraps __{@link readContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getCurrentBlockGasLimit"`
 */
export const readIMulticall3GetCurrentBlockGasLimit = /*#__PURE__*/ createReadContract({
  abi: iMulticall3Abi,
  functionName: 'getCurrentBlockGasLimit',
})

/**
 * Wraps __{@link readContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getCurrentBlockTimestamp"`
 */
export const readIMulticall3GetCurrentBlockTimestamp = /*#__PURE__*/ createReadContract({
  abi: iMulticall3Abi,
  functionName: 'getCurrentBlockTimestamp',
})

/**
 * Wraps __{@link readContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getEthBalance"`
 */
export const readIMulticall3GetEthBalance = /*#__PURE__*/ createReadContract({
  abi: iMulticall3Abi,
  functionName: 'getEthBalance',
})

/**
 * Wraps __{@link readContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getLastBlockHash"`
 */
export const readIMulticall3GetLastBlockHash = /*#__PURE__*/ createReadContract(
  { abi: iMulticall3Abi, functionName: 'getLastBlockHash' },
)

/**
 * Wraps __{@link writeContract}__ with `abi` set to __{@link iMulticall3Abi}__
 */
export const writeIMulticall3 = /*#__PURE__*/ createWriteContract({
  abi: iMulticall3Abi,
})

/**
 * Wraps __{@link writeContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"aggregate"`
 */
export const writeIMulticall3Aggregate = /*#__PURE__*/ createWriteContract({
  abi: iMulticall3Abi,
  functionName: 'aggregate',
})

/**
 * Wraps __{@link writeContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"aggregate3"`
 */
export const writeIMulticall3Aggregate3 = /*#__PURE__*/ createWriteContract({
  abi: iMulticall3Abi,
  functionName: 'aggregate3',
})

/**
 * Wraps __{@link writeContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"aggregate3Value"`
 */
export const writeIMulticall3Aggregate3Value = /*#__PURE__*/ createWriteContract({
  abi: iMulticall3Abi,
  functionName: 'aggregate3Value',
})

/**
 * Wraps __{@link writeContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"blockAndAggregate"`
 */
export const writeIMulticall3BlockAndAggregate = /*#__PURE__*/ createWriteContract({
  abi: iMulticall3Abi,
  functionName: 'blockAndAggregate',
})

/**
 * Wraps __{@link writeContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"tryAggregate"`
 */
export const writeIMulticall3TryAggregate = /*#__PURE__*/ createWriteContract({
  abi: iMulticall3Abi,
  functionName: 'tryAggregate',
})

/**
 * Wraps __{@link writeContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"tryBlockAndAggregate"`
 */
export const writeIMulticall3TryBlockAndAggregate = /*#__PURE__*/ createWriteContract({
  abi: iMulticall3Abi,
  functionName: 'tryBlockAndAggregate',
})

/**
 * Wraps __{@link simulateContract}__ with `abi` set to __{@link iMulticall3Abi}__
 */
export const simulateIMulticall3 = /*#__PURE__*/ createSimulateContract({
  abi: iMulticall3Abi,
})

/**
 * Wraps __{@link simulateContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"aggregate"`
 */
export const simulateIMulticall3Aggregate = /*#__PURE__*/ createSimulateContract({
  abi: iMulticall3Abi,
  functionName: 'aggregate',
})

/**
 * Wraps __{@link simulateContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"aggregate3"`
 */
export const simulateIMulticall3Aggregate3 = /*#__PURE__*/ createSimulateContract({
  abi: iMulticall3Abi,
  functionName: 'aggregate3',
})

/**
 * Wraps __{@link simulateContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"aggregate3Value"`
 */
export const simulateIMulticall3Aggregate3Value = /*#__PURE__*/ createSimulateContract({
  abi: iMulticall3Abi,
  functionName: 'aggregate3Value',
})

/**
 * Wraps __{@link simulateContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"blockAndAggregate"`
 */
export const simulateIMulticall3BlockAndAggregate = /*#__PURE__*/ createSimulateContract({
  abi: iMulticall3Abi,
  functionName: 'blockAndAggregate',
})

/**
 * Wraps __{@link simulateContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"tryAggregate"`
 */
export const simulateIMulticall3TryAggregate = /*#__PURE__*/ createSimulateContract({
  abi: iMulticall3Abi,
  functionName: 'tryAggregate',
})

/**
 * Wraps __{@link simulateContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"tryBlockAndAggregate"`
 */
export const simulateIMulticall3TryBlockAndAggregate = /*#__PURE__*/ createSimulateContract({
  abi: iMulticall3Abi,
  functionName: 'tryBlockAndAggregate',
})

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// React
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/**
 * Wraps __{@link useReadContract}__ with `abi` set to __{@link counterAbi}__
 *
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const useReadCounter = /*#__PURE__*/ createUseReadContract({
  abi: counterAbi,
  address: counterAddress,
})

/**
 * Wraps __{@link useReadContract}__ with `abi` set to __{@link counterAbi}__ and `functionName` set to `"number"`
 *
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const useReadCounterNumber = /*#__PURE__*/ createUseReadContract({
  abi: counterAbi,
  address: counterAddress,
  functionName: 'number',
})

/**
 * Wraps __{@link useWriteContract}__ with `abi` set to __{@link counterAbi}__
 *
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const useWriteCounter = /*#__PURE__*/ createUseWriteContract({
  abi: counterAbi,
  address: counterAddress,
})

/**
 * Wraps __{@link useWriteContract}__ with `abi` set to __{@link counterAbi}__ and `functionName` set to `"increment"`
 *
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const useWriteCounterIncrement = /*#__PURE__*/ createUseWriteContract({
  abi: counterAbi,
  address: counterAddress,
  functionName: 'increment',
})

/**
 * Wraps __{@link useWriteContract}__ with `abi` set to __{@link counterAbi}__ and `functionName` set to `"setNumber"`
 *
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const useWriteCounterSetNumber = /*#__PURE__*/ createUseWriteContract({
  abi: counterAbi,
  address: counterAddress,
  functionName: 'setNumber',
})

/**
 * Wraps __{@link useSimulateContract}__ with `abi` set to __{@link counterAbi}__
 *
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const useSimulateCounter = /*#__PURE__*/ createUseSimulateContract({
  abi: counterAbi,
  address: counterAddress,
})

/**
 * Wraps __{@link useSimulateContract}__ with `abi` set to __{@link counterAbi}__ and `functionName` set to `"increment"`
 *
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const useSimulateCounterIncrement = /*#__PURE__*/ createUseSimulateContract({
  abi: counterAbi,
  address: counterAddress,
  functionName: 'increment',
})

/**
 * Wraps __{@link useSimulateContract}__ with `abi` set to __{@link counterAbi}__ and `functionName` set to `"setNumber"`
 *
 * -
 * - [__View Contract on Sepolia Etherscan__](https://sepolia.etherscan.io/address/0xFd4364354145989210ee6d93a36B4eC7D8eC65EA)
 */
export const useSimulateCounterSetNumber = /*#__PURE__*/ createUseSimulateContract({
  abi: counterAbi,
  address: counterAddress,
  functionName: 'setNumber',
})

/**
 * Wraps __{@link useReadContract}__ with `abi` set to __{@link groth16VerifierAbi}__
 */
export const useReadGroth16Verifier = /*#__PURE__*/ createUseReadContract({
  abi: groth16VerifierAbi,
})

/**
 * Wraps __{@link useReadContract}__ with `abi` set to __{@link groth16VerifierAbi}__ and `functionName` set to `"verifyProof"`
 */
export const useReadGroth16VerifierVerifyProof = /*#__PURE__*/ createUseReadContract({
  abi: groth16VerifierAbi,
  functionName: 'verifyProof',
})

/**
 * Wraps __{@link useReadContract}__ with `abi` set to __{@link iMulticall3Abi}__
 */
export const useReadIMulticall3 = /*#__PURE__*/ createUseReadContract({
  abi: iMulticall3Abi,
})

/**
 * Wraps __{@link useReadContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getBasefee"`
 */
export const useReadIMulticall3GetBasefee = /*#__PURE__*/ createUseReadContract(
  { abi: iMulticall3Abi, functionName: 'getBasefee' },
)

/**
 * Wraps __{@link useReadContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getBlockHash"`
 */
export const useReadIMulticall3GetBlockHash = /*#__PURE__*/ createUseReadContract({
  abi: iMulticall3Abi,
  functionName: 'getBlockHash',
})

/**
 * Wraps __{@link useReadContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getBlockNumber"`
 */
export const useReadIMulticall3GetBlockNumber = /*#__PURE__*/ createUseReadContract({
  abi: iMulticall3Abi,
  functionName: 'getBlockNumber',
})

/**
 * Wraps __{@link useReadContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getChainId"`
 */
export const useReadIMulticall3GetChainId = /*#__PURE__*/ createUseReadContract(
  { abi: iMulticall3Abi, functionName: 'getChainId' },
)

/**
 * Wraps __{@link useReadContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getCurrentBlockCoinbase"`
 */
export const useReadIMulticall3GetCurrentBlockCoinbase = /*#__PURE__*/ createUseReadContract({
  abi: iMulticall3Abi,
  functionName: 'getCurrentBlockCoinbase',
})

/**
 * Wraps __{@link useReadContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getCurrentBlockDifficulty"`
 */
export const useReadIMulticall3GetCurrentBlockDifficulty = /*#__PURE__*/ createUseReadContract({
  abi: iMulticall3Abi,
  functionName: 'getCurrentBlockDifficulty',
})

/**
 * Wraps __{@link useReadContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getCurrentBlockGasLimit"`
 */
export const useReadIMulticall3GetCurrentBlockGasLimit = /*#__PURE__*/ createUseReadContract({
  abi: iMulticall3Abi,
  functionName: 'getCurrentBlockGasLimit',
})

/**
 * Wraps __{@link useReadContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getCurrentBlockTimestamp"`
 */
export const useReadIMulticall3GetCurrentBlockTimestamp = /*#__PURE__*/ createUseReadContract({
  abi: iMulticall3Abi,
  functionName: 'getCurrentBlockTimestamp',
})

/**
 * Wraps __{@link useReadContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getEthBalance"`
 */
export const useReadIMulticall3GetEthBalance = /*#__PURE__*/ createUseReadContract({
  abi: iMulticall3Abi,
  functionName: 'getEthBalance',
})

/**
 * Wraps __{@link useReadContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"getLastBlockHash"`
 */
export const useReadIMulticall3GetLastBlockHash = /*#__PURE__*/ createUseReadContract({
  abi: iMulticall3Abi,
  functionName: 'getLastBlockHash',
})

/**
 * Wraps __{@link useWriteContract}__ with `abi` set to __{@link iMulticall3Abi}__
 */
export const useWriteIMulticall3 = /*#__PURE__*/ createUseWriteContract({
  abi: iMulticall3Abi,
})

/**
 * Wraps __{@link useWriteContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"aggregate"`
 */
export const useWriteIMulticall3Aggregate = /*#__PURE__*/ createUseWriteContract({
  abi: iMulticall3Abi,
  functionName: 'aggregate',
})

/**
 * Wraps __{@link useWriteContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"aggregate3"`
 */
export const useWriteIMulticall3Aggregate3 = /*#__PURE__*/ createUseWriteContract({
  abi: iMulticall3Abi,
  functionName: 'aggregate3',
})

/**
 * Wraps __{@link useWriteContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"aggregate3Value"`
 */
export const useWriteIMulticall3Aggregate3Value = /*#__PURE__*/ createUseWriteContract({
  abi: iMulticall3Abi,
  functionName: 'aggregate3Value',
})

/**
 * Wraps __{@link useWriteContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"blockAndAggregate"`
 */
export const useWriteIMulticall3BlockAndAggregate = /*#__PURE__*/ createUseWriteContract({
  abi: iMulticall3Abi,
  functionName: 'blockAndAggregate',
})

/**
 * Wraps __{@link useWriteContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"tryAggregate"`
 */
export const useWriteIMulticall3TryAggregate = /*#__PURE__*/ createUseWriteContract({
  abi: iMulticall3Abi,
  functionName: 'tryAggregate',
})

/**
 * Wraps __{@link useWriteContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"tryBlockAndAggregate"`
 */
export const useWriteIMulticall3TryBlockAndAggregate = /*#__PURE__*/ createUseWriteContract({
  abi: iMulticall3Abi,
  functionName: 'tryBlockAndAggregate',
})

/**
 * Wraps __{@link useSimulateContract}__ with `abi` set to __{@link iMulticall3Abi}__
 */
export const useSimulateIMulticall3 = /*#__PURE__*/ createUseSimulateContract({
  abi: iMulticall3Abi,
})

/**
 * Wraps __{@link useSimulateContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"aggregate"`
 */
export const useSimulateIMulticall3Aggregate = /*#__PURE__*/ createUseSimulateContract({
  abi: iMulticall3Abi,
  functionName: 'aggregate',
})

/**
 * Wraps __{@link useSimulateContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"aggregate3"`
 */
export const useSimulateIMulticall3Aggregate3 = /*#__PURE__*/ createUseSimulateContract({
  abi: iMulticall3Abi,
  functionName: 'aggregate3',
})

/**
 * Wraps __{@link useSimulateContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"aggregate3Value"`
 */
export const useSimulateIMulticall3Aggregate3Value = /*#__PURE__*/ createUseSimulateContract({
  abi: iMulticall3Abi,
  functionName: 'aggregate3Value',
})

/**
 * Wraps __{@link useSimulateContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"blockAndAggregate"`
 */
export const useSimulateIMulticall3BlockAndAggregate = /*#__PURE__*/ createUseSimulateContract({
  abi: iMulticall3Abi,
  functionName: 'blockAndAggregate',
})

/**
 * Wraps __{@link useSimulateContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"tryAggregate"`
 */
export const useSimulateIMulticall3TryAggregate = /*#__PURE__*/ createUseSimulateContract({
  abi: iMulticall3Abi,
  functionName: 'tryAggregate',
})

/**
 * Wraps __{@link useSimulateContract}__ with `abi` set to __{@link iMulticall3Abi}__ and `functionName` set to `"tryBlockAndAggregate"`
 */
export const useSimulateIMulticall3TryBlockAndAggregate = /*#__PURE__*/ createUseSimulateContract({
  abi: iMulticall3Abi,
  functionName: 'tryBlockAndAggregate',
})
