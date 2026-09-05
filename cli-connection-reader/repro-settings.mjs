export const READER_BUILD_SETTINGS = Object.freeze({
  bundle: Object.freeze({ platform: 'node', format: 'cjs', target: 'node24' }),
  sea: Object.freeze({
    disableExperimentalWarning: true,
    section: 'NODE_SEA_BLOB',
    sentinelFuse: 'NODE_SEA_FUSE_fce680ab2cc467b6e072b8b5df1996b2',
  }),
});
