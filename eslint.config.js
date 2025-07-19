import antfu from '@antfu/eslint-config'

export default antfu({
  ignores: ['src-tauri/src/**/*', 'node_modules/**/*', 'src-tauri/target/**/*'],
})
