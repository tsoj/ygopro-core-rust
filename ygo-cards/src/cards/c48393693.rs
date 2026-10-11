//! Gagaga Girl - Cell Phone Subtraction
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 48393693;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --If your opponent Special Summons a monster(s) from the Extra Deck: You can target 1 face-up monster you control; Special Summon this card from your hand, then immediately after this effect resolves, Xyz Summon using only this card and the targeted monster. When you do, treat this card's Level as the same as the targeted monster's
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --If this card is detached from an Xyz Monster to activate that monster's effect: You can change the ATK of 1 face-up monster your opponent controls to 0
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2a, CATEGORY_ATKCHANGE);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    // e2a:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2a, EVENT_TO_GRAVE);
    // e2a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    // e2a:SetCondition(s.atkcon)
    effect::set_condition(d, e2a, Some(condition(atkcon)));
    // e2a:SetTarget(s.atktg)
    effect::set_target(d, e2a, Some(cost_target(atktg)));
    // e2a:SetOperation(s.atkop)
    effect::set_operation(d, e2a, Some(operation(atkop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2b, EVENT_REMOVE);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `s.spconfilter(c,opp)`
fn spconfilter(d: &mut Duel, c: CardId, opp: u8) -> bool {
    // return c:IsSummonPlayer(opp) and c:IsSummonLocation(LOCATION_EXTRA)
    card::is_summon_player(d, c, opp) && card::is_summon_location(d, c, LOCATION_EXTRA)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return eg:IsExists(s.spconfilter,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, x| spconfilter(d, x, 1 - tp), 1, NO_EX)
}
/// `s.tgfilter(c,tp,hc)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8, hc: CardId) -> bool {
    // if not (c:IsFaceup() and c:HasLevel()) then return false end
    if !(card::is_faceup(d, c) && card::has_level(d, c)) {
        return false;
    }
    // local e1=Effect.CreateEffect(hc)
    let e1 = effect::create_effect(d, hc);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_XYZ_LEVEL)
    effect::set_code(d, e1, EFFECT_XYZ_LEVEL);
    // e1:SetValue(c:GetLevel())
    let lv = card::get_level(d, c);
    effect::set_value(d, e1, lv as i64);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // hc:RegisterEffect(e1,true)
    card::register_effect(d, hc, e1, true);
    // local res=Duel.IsExistingMatchingCard(Card.IsXyzSummonable,tp,LOCATION_EXTRA,0,1,nil,nil,Group.FromCards(c,hc),2,2)
    let mg = CardSet::from_card(c);
    let mut mg = mg;
    group::add_card(&mut mg, &CardSet::from_card(hc));
    let res = duel::is_existing_matching_card(d, Some(&|d, x| card::is_xyz_summonable(d, x, None, Some(mg.clone()), 2, 2)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    // if e1 then e1:Reset() end
    effect::reset(d, e1);
    // return res
    res
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.tgfilter(chkc,tp,c) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && tgfilter(d, chkc, tp, c);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and Duel.IsPlayerCanSpecialSummonCount(tp,2)
    //   and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,tp,c) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
            && duel::is_player_can_special_summon_count(d, tp, 2)
            && duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, tp, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,tp,c)
    let _g = duel::select_target(d, tp, Some(&|d, x| tgfilter(d, x, tp, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, c, e)
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && card::is_relate_to_effect(d, tc, e)
        && card::is_faceup(d, tc)
    {
        // local mg=Group.FromCards(c,tc)
        let mut mg = CardSet::from_card(c);
        group::add_card(&mut mg, &CardSet::from_card(tc));
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_XYZ_LEVEL)
        effect::set_code(d, e1, EFFECT_XYZ_LEVEL);
        // e1:SetValue(tc:GetLevel())
        let lv = card::get_level(d, tc);
        effect::set_value(d, e1, lv as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local xyz=Duel.SelectMatchingCard(tp,Card.IsXyzSummonable,tp,LOCATION_EXTRA,0,1,1,nil,nil,mg,2,2):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, x| card::is_xyz_summonable(d, x, None, Some(mg.clone()), 2, 2)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        let xyz = group::get_first(d, &sel);
        // if xyz then
        if let Some(xyz) = xyz {
            // Duel.XyzSummon(tp,xyz,mg,nil,2,2)
            duel::xyz_summon(d, tp, xyz, Some(mg), None, 2, 2);
        } else {
            // if e1 then e1:Reset() end
            effect::reset(d, e1);
        }
    }
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_COST) and re:IsActivated() and re:IsActiveType(TYPE_XYZ) and c:IsPreviousLocation(LOCATION_OVERLAY)
    card::is_reason(d, c, REASON_COST)
        && effect::is_activated(d, a.re.nil())
        && effect::is_active_type(d, a.re.nil(), TYPE_XYZ)
        && card::is_previous_location(d, c, LOCATION_OVERLAY)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.HasNonZeroAttack,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::has_non_zero_attack), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,nil,1,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, None, 1, 1 - tp, 0);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
    // local sc=Duel.SelectMatchingCard(tp,Card.HasNonZeroAttack,tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&card::has_non_zero_attack), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc then
    if let Some(sc) = sc {
        // Duel.HintSelection(sc)
        duel::hint_selection(d, sc, true);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let c = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
    }
}
