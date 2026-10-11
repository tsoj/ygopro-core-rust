//! Ancient City - Rainbow Ruins
use super::prelude::*;

pub const ID: u32 = 34487429;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_CRYSTAL_BEAST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Prevent destruction by effects
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_condition(d, e2, Some(lua_fn(desrepcon)));
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    // Halves battle damage
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_code(d, e3, EVENT_PRE_BATTLE_DAMAGE);
    effect::set_condition(d, e3, Some(condition(rdcon)));
    effect::set_operation(d, e3, Some(operation(rdop)));
    card::register_effect(d, c, e3, false);
    // Negate Spell/Trap activation
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_NEGATE | CATEGORY_DESTROY);
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e4, EVENT_CHAINING);
    effect::set_range(d, e4, LOCATION_FZONE);
    effect::set_condition(d, e4, Some(condition(discon)));
    effect::set_cost(d, e4, Some(cost_target(discost)));
    effect::set_target(d, e4, Some(cost_target(distg)));
    effect::set_operation(d, e4, Some(operation(disop)));
    card::register_effect(d, c, e4, false);
    // Draw 1 card
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 2));
    effect::set_category(d, e5, CATEGORY_DRAW);
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e5, LOCATION_FZONE);
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    effect::set_condition(d, e5, Some(condition(drcon)));
    effect::set_target(d, e5, Some(cost_target(drtg)));
    effect::set_operation(d, e5, Some(operation(drop_op)));
    card::register_effect(d, c, e5, false);
    // Special Summon 1 "Crystal Beast" monster from the Spell/Trap Zone
    let e6 = effect::create_effect(d, c);
    effect::set_description(d, e6, aux::stringid(ID, 3));
    effect::set_category(d, e6, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e6, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e6, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e6, LOCATION_FZONE);
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    effect::set_condition(d, e6, Some(condition(spcon)));
    effect::set_target(d, e6, Some(cost_target(sptg)));
    effect::set_operation(d, e6, Some(operation(spop)));
    card::register_effect(d, c, e6, false);
    aux::double_snare_validity(d, Some(c), LOCATION_SZONE, None);
}
// s.desrepcon(e)
fn desrepcon(d: &mut Duel, a: Args) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_CRYSTAL_BEAST),e:GetHandler():GetControler(),LOCATION_SZONE,0,1,nil)
    let h = effect::get_handler(d, a.e(0)).nil();
    let p = card::get_controler(d, h);
    let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]));
    duel::is_existing_matching_card(d, Some(&f), p, LOCATION_SZONE, 0, 1, NO_EX)
}
// s.rdcon
fn rdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp and e:GetHandler():GetFlagEffect(id)==0
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_CRYSTAL_BEAST),tp,LOCATION_SZONE,0,2,nil)
    if a.ep != a.tp {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    if card::get_flag_effect(d, h, ID) != 0 {
        return false;
    }
    let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]));
    duel::is_existing_matching_card(d, Some(&f), a.tp, LOCATION_SZONE, 0, 2, NO_EX)
}
// s.rdop
fn rdop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    // if Duel.SelectEffectYesNo(tp,e:GetHandler()) then
    if duel::select_effect_yes_no(d, a.tp, h, None) {
        // Duel.Hint(HINT_CARD,1-tp,id)
        duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
        // Duel.ChangeBattleDamage(tp,math.floor(ev/2))
        duel::change_battle_damage(d, a.tp, (a.ev / 2) as u32, true);
        // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
// s.discon
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsChainNegatable(ev)
    //     and Duel.IsExistingMatchingCard(Card.IsSetCard,tp,LOCATION_SZONE,0,3,nil,SET_CRYSTAL_BEAST)
    let re = a.re.nil();
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && duel::is_chain_negatable(d, a.ev as u8)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16])), a.tp, LOCATION_SZONE, 0, 3, NO_EX)
}
// s.filter2(c)
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]) && card::is_able_to_grave_as_cost(d, c)
}
// s.discost
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter2), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter2), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
// s.distg
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
// s.disop
fn disop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_CRYSTAL_BEAST),tp,LOCATION_SZONE,0,3,nil) then return end
    let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]));
    if !duel::is_existing_matching_card(d, Some(&f), a.tp, LOCATION_SZONE, 0, 3, NO_EX) {
        return;
    }
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
// s.drcon
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_CRYSTAL_BEAST),tp,LOCATION_SZONE,0,4,nil)
    let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]));
    duel::is_existing_matching_card(d, Some(&f), a.tp, LOCATION_SZONE, 0, 4, NO_EX)
}
// s.drtg
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
// s.drop
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_CRYSTAL_BEAST),tp,LOCATION_SZONE,0,4,nil) then return end
    let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]));
    if !duel::is_existing_matching_card(d, Some(&f), a.tp, LOCATION_SZONE, 0, 4, NO_EX) {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
// s.spcon
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_CRYSTAL_BEAST),tp,LOCATION_SZONE,0,5,nil)
    let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]));
    duel::is_existing_matching_card(d, Some(&f), a.tp, LOCATION_SZONE, 0, 5, NO_EX)
}
// s.filter3(c,e,tp)
fn filter3(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// s.sptg
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and chkc:IsControler(tp) and s.filter3(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && card::is_controler(d, chkc, a.tp) && filter3(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter3,tp,LOCATION_SZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter3(d, c, a.e, a.tp)), a.tp, LOCATION_SZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter3,tp,LOCATION_SZONE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter3(d, c, a.e, a.tp)), a.tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
// s.spop
fn spop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_CRYSTAL_BEAST),tp,LOCATION_SZONE,0,5,nil) then return end
    let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]));
    if !duel::is_existing_matching_card(d, Some(&f), a.tp, LOCATION_SZONE, 0, 5, NO_EX) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,true,false,POS_FACEUP)
            duel::special_summon(d, CardSet::from_card(tc), 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
        }
    }
}
