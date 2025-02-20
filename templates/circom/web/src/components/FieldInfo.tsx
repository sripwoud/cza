import type { FieldApi } from '@tanstack/react-form'

// biome-ignore lint/suspicious/noExplicitAny: This is a generic component
export function FieldInfo({ field }: { field: FieldApi<any, any, any, any> }) {
  return (
    <>
      {field.state.meta.isTouched && field.state.meta.errors.length
        ? <em style={{ color: 'red' }}>{field.state.meta.errors.join(', ')}</em>
        : null}
    </>
  )
}
