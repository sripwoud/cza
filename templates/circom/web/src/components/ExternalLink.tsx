import type { FC, ReactNode } from 'react'

interface ExternalLinkProps {
  children: ReactNode
  className?: string
  href: string
}

export const ExternalLink: FC<ExternalLinkProps> = ({
  children,
  href,
}: ExternalLinkProps) => (
  <a href={href} rel='noopener noreferrer' target='_blank'>
    {children}
  </a>
)
