# program_derived_addresses_seahorse
# Built with Seahorse v0.2.0

from seahorse.prelude import *

declare_id('3AZUzzM9zVoeW1gmkjUYaWWGvHSvmmsajtzJLcgGcEQP')

class PageVisits(Account):
  visits: u32

@instruction
def create_page_visits(owner: Signer, page_visits: Empty[PageVisits]):
  page_visits.init(payer = owner, seeds = ['page_visits', owner])

@instruction
def increment_page_visits(page_visits: PageVisits):
  page_visits.visits += 1
