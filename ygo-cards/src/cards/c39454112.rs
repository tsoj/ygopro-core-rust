//! That Six
use super::prelude::*;

pub const ID: u32 = 39454112;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change die result
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_TOSS_DICE_NEGATE)
    effect::set_code(d, e2, EVENT_TOSS_DICE_NEGATE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetOperation(s.diceop)
    effect::set_operation(d, e2, Some(operation(diceop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.diceop(e,tp,eg,ep,ev,re,r,rp)`
fn diceop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local cc=Duel.GetCurrentChain()
    let cc = duel::get_current_chain(d, false);
    // local cid=Duel.GetChainInfo(cc,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, cc as u8, &[CHAININFO_CHAIN_ID]).first().map_or(0, |v| v.int());
    // if s[0]~=cid and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    let already = matches!(s_var(d, ID, 0), Value::Int(x) if x == cid);
    if !already && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // local dc={Duel.GetDiceResult()}
        let mut dc: Vec<i32> = duel::get_dice_result(d);
        // local ac=1
        let mut ac: usize = 1;
        // local ct=(ev&0xff)+(ev>>16)
        let ev = a.ev as u64;
        let ct = ((ev & 0xff) + (ev >> 16)) as usize;
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // if ct>1 then
        if ct > 1 {
            // local val,idx=Duel.AnnounceNumber(tp,table.unpack(dc,1,ct))
            let nums: Vec<u64> = dc.iter().take(ct).map(|&x| x as u64).collect();
            let (_val, idx) = duel::announce_number(d, tp, &nums).nil();
            // ac=idx+1
            ac = idx as usize + 1;
        }
        // if dc[ac]==1 or dc[ac]==3 or dc[ac]==5 then dc[ac]=6
        // else dc[ac]=1 end
        let v = dc[ac - 1];
        dc[ac - 1] = if v == 1 || v == 3 || v == 5 { 6 } else { 1 };
        // Duel.SetDiceResult(table.unpack(dc))
        let results: Vec<u8> = dc.iter().map(|&x| x as u8).collect();
        duel::set_dice_result(d, &results);
        // s[0]=cid
        set_s_var(d, ID, 0, Value::Int(cid));
    }
}
