//! Vanquish Soul Calamity Caesar
//! Scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 80738884;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VANQUISH_SOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,p)`
fn filter(d: &mut Duel, c: CardId, p: u8) -> bool {
    // return c:IsOnField() and c:IsControler(p)
    card::is_on_field(d, c) && card::is_controler(d, c, p)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if rp==tp or not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_VANQUISH_SOUL),tp,LOCATION_MZONE,0,1,nil) then return false end
    if a.rp == tp
        || !duel::is_existing_matching_card(
            d,
            Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16]))),
            tp,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        )
    {
        return false;
    }
    let re = a.re.nil();
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let chain = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    // return tg and tg:IsExists(s.filter,1,nil,tp) and Duel.IsChainNegatable(ev)
    //     and (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE))
    let Some(tg) = chain[0].as_group().cloned() else {
        return false;
    };
    if !group::is_exists(d, &tg, &|d, c| filter(d, c, tp), 1, NO_EX) {
        return false;
    }
    if !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    effect::is_monster_effect(d, re) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
        duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 0);
    }
    true
}
/// `s.damfilter(c)`
fn damfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VANQUISH_SOUL) and c:GetAttack()>0
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16]) && card::get_attack(d, c) > 0
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let re = a.re.nil();
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) and Duel.Destroy(eg,REASON_EFFECT)>0 then
    if !duel::negate_activation(d, a.ev as u8) {
        return;
    }
    let rc = effect::get_handler(d, re).nil();
    if !card::is_relate_to_effect(d, rc, re) {
        return;
    }
    let eg = a.eg.clone().nil();
    if duel::destroy(d, eg, REASON_EFFECT, None, None) <= 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.damfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&damfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if g.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local tc=g:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // Duel.HintSelection(tc,true)
    duel::hint_selection(d, CardSet::from_card(tc), true);
    // local dam=tc:GetAttack()
    let dam = card::get_attack(d, tc) as i64;
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.Damage(1-tp,dam,REASON_EFFECT)
    duel::damage(d, 1 - tp, dam, REASON_EFFECT, false, None);
}
