//! Link into the VRAINS!
use super::prelude::*;

pub const ID: u32 = 28827503;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon from the hand, then Link Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 monster from your GY to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter2(c,mc,fg)`
fn spfilter2(d: &mut Duel, c: CardId, mc: CardId, fg: &CardSet) -> bool {
    // return c:IsLinkSummonable(mc,fg+mc)
    let mut mats = fg.clone();
    group::add_card(&mut mats, &CardSet::from_card(mc));
    card::is_link_summonable(d, c, Some(CardSet::from_card(mc)), Some(mats), 0, 0)
}
/// `s.spfilter(c,e,tp,fg)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, fg: &CardSet) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_EXTRA,0,1,nil,c,fg)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter2(d, x, c, fg)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local fg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCanBeLinkMaterial),tp,LOCATION_MZONE,0,nil)
    let fg = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_can_be_link_material(d, c, None, None)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp,fg)
    //     and Duel.IsPlayerCanSpecialSummonCount(tp,2) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, &fg)), a.tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::is_player_can_special_summon_count(d, a.tp, 2);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_HAND|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, (LOCATION_HAND | LOCATION_EXTRA) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local fg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCanBeLinkMaterial),tp,LOCATION_MZONE,0,nil)
    let fg = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_can_be_link_material(d, c, None, None)), tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp,fg):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp, &fg)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if not tc or not Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then return false end
    let Some(tc) = tc else { return };
    if !duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Negate its effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_DISABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
    // e2:SetValue(RESET_TURN_SET)
    effect::set_value(d, e2, RESET_TURN_SET as i64);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // local tg=Duel.GetMatchingGroup(s.spfilter2,tp,LOCATION_EXTRA,0,nil,tc,fg)
    let tg = duel::get_matching_group(d, Some(&|d, x| spfilter2(d, x, tc, &fg)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=tg:Select(tp,1,1,nil)
        let sg = group::select(d, &tg, tp, 1, 1, false, NO_EX).nil();
        // local sc=sg:GetFirst()
        let sc = group::get_first(d, &sg).nil();
        // Link Summon cannot be negated
        // local e2=Effect.CreateEffect(c)
        let le2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, le2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_CANNOT_DISABLE_SPSUMMON)
        effect::set_code(d, le2, EFFECT_CANNOT_DISABLE_SPSUMMON);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
        effect::set_property(d, le2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
        // e2:SetCondition(s.effcon)
        effect::set_condition(d, le2, Some(condition(effcon)));
        // sc:RegisterEffect(e2)
        card::register_effect(d, sc, le2, false);
        // Opponent cannot activate cards/effects on Link Summon
        // local e3=Effect.CreateEffect(c)
        let le3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, le3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, le3, EVENT_SPSUMMON_SUCCESS);
        // e3:SetLabelObject(e2)
        effect::set_label_object(d, le3, Value::Effect(le2));
        // e3:SetCondition(s.effcon)
        effect::set_condition(d, le3, Some(condition(effcon)));
        // e3:SetOperation(s.spsumsuc)
        effect::set_operation(d, le3, Some(operation(spsumsuc)));
        // sc:RegisterEffect(e3)
        card::register_effect(d, sc, le3, false);
        // Duel.LinkSummon(tp,sc,tc,nil)
        duel::link_summon(d, tp, sc, Some(CardSet::from_card(tc)), None, 0, 0);
    }
}
/// `s.effcon(e)`
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
/// `s.spsumsuc(e,tp,eg,ep,ev,re,r,rp)`
fn spsumsuc(d: &mut Duel, a: &EffArgs) {
    // Duel.SetChainLimitTillChainEnd(s.chlimit)
    duel::set_chain_limit_till_chain_end(d, lua_fn(chlimit));
    // e:GetLabelObject():Reset()
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::reset(d, lo);
    // e:Reset()
    effect::reset(d, a.e);
}
/// `s.chlimit(e,ep,tp)`
fn chlimit(_d: &mut Duel, a: Args) -> bool {
    // return tp==ep
    a.p(2) == a.p(1)
}
/// `s.thcfilter(c,tp)`
fn thcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsReason(REASON_BATTLE|REASON_EFFECT) and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE)
    //     and c:IsPreviousPosition(POS_FACEUP) and c:GetPreviousTypeOnField() & TYPE_LINK ~=0
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,nil,c:GetOriginalRace())
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP)
        && (card::get_previous_type_on_field(d, c) & TYPE_LINK) != 0
        && {
            let rc = card::get_original_race(d, c);
            duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, rc)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        }
}
/// `s.thfilter(c,rc)`
fn thfilter(d: &mut Duel, c: CardId, rc: u64) -> bool {
    // return c:IsMonster() and c:IsOriginalRace(rc) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_original_race(d, c, rc) && card::is_able_to_hand(d, c, None)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not eg:IsContains(e:GetHandler()) and eg:IsExists(s.thcfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    !eg.contains(h) && group::is_exists(d, &eg, &|d, c| thcfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thfilter2(c,eg)`
fn thfilter2(d: &mut Duel, c: CardId, eg: &CardSet) -> bool {
    // return c:IsMonster() and c:IsAbleToHand() and eg:IsExists(Card.IsOriginalRace,1,nil,c:GetOriginalRace())
    let rc = card::get_original_race(d, c);
    card::is_monster(d, c) && card::is_able_to_hand(d, c, None) && group::is_exists(d, eg, &|d, x| card::is_original_race(d, x, rc), 1, NO_EX)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    // local dg=eg:Filter(s.thcfilter,nil,tp)
    let dg = group::filter(d, &eg, &|d, c| thcfilter(d, c, tp), NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter2),tp,LOCATION_GRAVE,0,1,1,nil,dg)
    let nv = aux::necro_valley_filter(|d, c| thfilter2(d, c, &dg));
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
