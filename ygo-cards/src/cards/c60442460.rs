//! Branded Befallen
//! 導かれし烙印
use super::prelude::*;

pub const ID: u32 = 60442460;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BYSTIAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Banish 1 LIGHT or DARK monster from either GY and negate an activated effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let re = a.re.nil();
    // if not (ep==1-tp and Duel.IsChainDisablable(ev)) or re:GetHandler():IsDisabled() then return false end
    if !(a.ep == 1 - tp && duel::is_chain_disablable(d, a.ev as u8)) {
        return false;
    }
    let rh = effect::get_handler(d, re).nil();
    if card::is_disabled(d, rh) {
        return false;
    }
    // local ch=Duel.GetCurrentChain(true)-1
    let ch = duel::get_current_chain(d, true) - 1;
    // if ch>0 then
    if ch > 0 {
        // local cplayer=Duel.GetChainInfo(ch,CHAININFO_TRIGGERING_CONTROLER)
        // local ceff=Duel.GetChainInfo(ch,CHAININFO_TRIGGERING_EFFECT)
        let info = duel::get_chain_info(d, ch as u8, &[CHAININFO_TRIGGERING_CONTROLER, CHAININFO_TRIGGERING_EFFECT]);
        let cplayer = info[0].int();
        // if cplayer==tp and ceff:GetHandler():IsSetCard(SET_BYSTIAL) and ceff:IsMonsterEffect() then
        if cplayer == tp as i64 {
            let ceff = info[1].as_effect().nil();
            let ch_h = effect::get_handler(d, ceff).nil();
            if card::is_set_card(d, ch_h, &[SET_BYSTIAL as u16]) && effect::is_monster_effect(d, ceff) {
                // return true
                return true;
            }
        }
    }
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    // if #g~=1 then return false end
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    if g.len() != 1 {
        return false;
    }
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // return tc:IsSetCard(SET_BYSTIAL) and tc:IsControler(tp) and tc:IsLocation(LOCATION_MZONE) and tc:IsFaceup()
    card::is_set_card(d, tc, &[SET_BYSTIAL as u16])
        && card::is_controler(d, tc, tp)
        && card::is_location(d, tc, LOCATION_MZONE)
        && card::is_faceup(d, tc)
}
/// `s.rmvfilter(c)`
fn rmvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK)
        && card::is_able_to_remove(d, c, None, None, None)
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and s.rmvfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && rmvfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmvfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmvfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmvfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&rmvfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)>0 then
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, e) && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, a.ev as u8);
    }
}
