//! Junoldo the Shadespirit Power Patron
use super::prelude::*;

pub const ID: u32 = 10266279;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, 5914858], listed_series: &[SET_POWER_PATRON, SET_ELFNOTE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // If a "Power Patron" and/or "Elfnote" monster(s) is Special Summoned to your field while this card is in the Pendulum Zone: You can destroy this card, and if you do, draw 2 cards, then discard 1 card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DRAW+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DRAW + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.pdescon)
    effect::set_condition(d, e1, Some(condition(pdescon)));
    // e1:SetTarget(s.pdestg)
    effect::set_target(d, e1, Some(cost_target(pdestg)));
    // e1:SetOperation(s.pdesop)
    effect::set_operation(d, e1, Some(operation(pdesop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can banish (face-down) the top 3 cards of your Deck; destroy this card, and if you do, Special Summon 1 "Junora the Power Patron of Tuning" from your Extra Deck (this is treated as a Synchro Summon)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.Replaceable(s.mdescost))
    effect::set_cost(d, e2, Some(cost::replaceable(cost_target(mdescost), None)));
    // e2:SetTarget(s.mdestg)
    effect::set_target(d, e2, Some(cost_target(mdestg)));
    // e2:SetOperation(s.mdesop)
    effect::set_operation(d, e2, Some(operation(mdesop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card is added to your Extra Deck face-up: You can add 1 "Power Patron" or "Elfnote" card from your face-up Extra Deck or GY to your hand, except "Power Patron Shadow Spirit Junordo"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_DECK)
    effect::set_code(d, e3, EVENT_TO_DECK);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.thcon)
    effect::set_condition(d, e3, Some(condition(thcon)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.pdesconfilter(c,tp)
fn pdesconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and c:IsSetCard({SET_POWER_PATRON,SET_ELFNOTE})
    card::is_faceup(d, c) && card::is_controler(d, c, tp) && card::is_set_card(d, c, &[SET_POWER_PATRON as u16, SET_ELFNOTE as u16])
}
// function s.pdescon(e,tp,eg,ep,ev,re,r,rp)
fn pdescon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return eg:IsExists(s.pdesconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| pdesconfilter(d, c, tp), 1, NO_EX)
}
// function s.pdestg(e,tp,eg,ep,ev,re,r,rp,chk)
fn pdestg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 2);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    true
}
// function s.pdesop(e,tp,eg,ep,ev,re,r,rp)
fn pdesop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.Destroy(c,REASON_EFFECT)>0 and Duel.Draw(tp,2,REASON_EFFECT)==2 then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::destroy(d, c, REASON_EFFECT, None, None) > 0
        && duel::draw(d, tp, 2, REASON_EFFECT) == 2
    {
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD)
        duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
// function s.junoraspfilter(c,e,tp,hc)
fn junoraspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, hc: Option<CardId>) -> bool {
    // return c:IsCode(5914858) and Duel.GetLocationCountFromEx(tp,tp,hc,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    card::is_code(d, c, &[5914858])
        && duel::get_location_count_from_ex(d, tp, Some(tp), hc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO as u32, tp, false, false, None, None, None)
}
// function s.mdescost(e,tp,eg,ep,ev,re,r,rp,chk)
fn mdescost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetDecktopGroup(tp,3)
    let g = duel::get_decktop_group(d, tp, 3);
    // if chk==0 then return g:IsExists(Card.IsAbleToRemoveAsCost,3,nil,POS_FACEDOWN) end
    if a.chk == Some(0) {
        return group::is_exists(d, &g, &|d, c| card::is_able_to_remove_as_cost(d, c, Some(POS_FACEDOWN as u8)), 3, NO_EX);
    }
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // Duel.Remove(g,POS_FACEDOWN,REASON_COST)
    duel::remove(d, g, POS_FACEDOWN as u8, REASON_COST, None, None);
    true
}
// function s.mdestg(e,tp,eg,ep,ev,re,r,rp,chk)
fn mdestg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.junoraspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| junoraspfilter(d, x, a.e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.mdesop(e,tp,eg,ep,ev,re,r,rp)
fn mdesop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.Destroy(c,REASON_EFFECT)==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) || duel::destroy(d, c, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.junoraspfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let sg = duel::select_matching_card(d, tp, Some(&|d, x| junoraspfilter(d, x, a.e, tp, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = match group::get_first(d, &sg) {
        Some(sc) => sc,
        // if not sc then return end
        None => return,
    };
    // sc:SetMaterial(nil)
    card::set_material(d, sc, CardSet::new());
    // if Duel.SpecialSummon(sc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)>0 then
    if duel::special_summon(d, sc, SUMMON_TYPE_SYNCHRO as u32, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard({SET_POWER_PATRON,SET_ELFNOTE}) and c:IsAbleToHand() and not c:IsCode(id)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_POWER_PATRON as u16, SET_ELFNOTE as u16])
        && card::is_able_to_hand(d, c, None)
        && !card::is_code(d, c, &[ID])
}
// function s.thcon(e)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsLocation(LOCATION_EXTRA) and c:IsFaceup()
    card::is_location(d, c, LOCATION_EXTRA) && card::is_faceup(d, c)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_EXTRA|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_EXTRA | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_EXTRA|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_EXTRA | LOCATION_GRAVE) as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_EXTRA|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, x| thfilter(d, x))),
        tp,
        LOCATION_EXTRA | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
