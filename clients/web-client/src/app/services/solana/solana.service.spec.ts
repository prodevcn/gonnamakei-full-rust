import { TestBed } from '@angular/core/testing';

import { SolanaService } from './solana.service';

describe('SolanaService', () => {
  let service: SolanaService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(SolanaService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
