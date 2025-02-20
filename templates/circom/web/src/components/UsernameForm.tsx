import { useForm } from '@tanstack/react-form'
import { FieldInfo } from 'c/FieldInfo'
import { capitalize } from 'l/format'
import { type UsernameSchema, usernameSchema } from 'l/schemas'
import type { FormEvent } from 'react'

export function UsernameForm() {
  const form = useForm<UsernameSchema>({
    defaultValues: {
      username: '',
    } as UsernameSchema,
    onSubmit: async ({ value: { username } }) => {
      alert(`Submitted - Username: ${username}`)
    },
    validators: { onChange: usernameSchema },
  })

  function handleSubmit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault()
    e.stopPropagation()
    form.handleSubmit()
  }

  return (
    <form onSubmit={(e) => handleSubmit(e)}>
      <form.Field
        name='username'
        children={(field) => (
          <>
            <label htmlFor={field.name} style={{ marginRight: '4px' }}>
              {capitalize(field.name)}
            </label>
            <input
              onChange={(e) => field.handleChange(e.target.value)}
              id={field.name}
              name={field.name}
              type='text'
              placeholder={field.name}
              value={field.state.value}
              style={{ marginRight: '4px' }}
            />
            <br />
            <FieldInfo field={field} />
            <br />
          </>
        )}
      />
      <form.Subscribe
        selector={({ canSubmit, isSubmitting }) => [canSubmit, isSubmitting]}
        children={([canSubmit, isSubmitting]) => (
          <button type='submit' aria-busy={isSubmitting} disabled={!canSubmit}>
            Submit
          </button>
        )}
      />
    </form>
  )
}
