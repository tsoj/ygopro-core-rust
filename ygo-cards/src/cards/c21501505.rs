//! Cairngorgon, Antiluminescent Knight
use super::prelude::*;

pub const ID: u32 = 21501505;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // replace
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if e==re or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if a.re == Some(a.e) || !effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].clone();
    // if not g or #g~=1 then return false end
    let Some(g) = g.as_group().cloned() else { return false };
    if g.len() != 1 {
        return false;
    }
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // e:SetLabelObject(tc)
    effect::set_label_object(d, a.e, Value::Card(tc));
    // return tc:IsOnField()
    card::is_on_field(d, tc)
}
/// `s.filter(c,ct)`
fn filter(d: &mut Duel, c: CardId, ct: i64) -> bool {
    // return Duel.CheckChainTarget(ct,c)
    duel::check_chain_target(d, ct as u8, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=ev
    let mut ct = a.ev as i64;
    // local label=Duel.GetFlagEffectLabel(0,id)
    let label = duel::get_flag_effect_label(d, 0, ID).first().copied();
    // if label then
    if let Some(label) = label {
        // if ev==(label>>16) then ct=(label&0xffff) end
        if a.ev as i64 == (label >> 16) {
            ct = label & 0xffff;
        }
    }
    // if chkc then return chkc:IsOnField() and s.filter(chkc,ct) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && filter(d, chkc, ct);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetLabelObject(),ct) end
    let lo = effect::get_label_object(d, a.e).as_card();
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, ct)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, lo.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,e:GetLabelObject(),ct)
    duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, ct)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, lo.into());
    // local val=ct+(ev+1<<16)
    let val = ct + ((a.ev as i64 + 1) << 16);
    // if label then
    if label.is_some() {
        // Duel.SetFlagEffectLabel(0,21501505,val)
        duel::set_flag_effect_label(d, 0, ID, val as i32);
    } else {
        // Duel.RegisterFlagEffect(0,21501505,RESET_CHAIN,0,1,val)
        duel::register_flag_effect(d, 0, ID, RESET_CHAIN, 0, 1, val as u32);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.ChangeTargetCard(ev,Group.FromCards(tc))
            duel::change_target_card(d, a.ev as u8, &CardSet::from_card(tc));
        }
    }
}
