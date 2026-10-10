//! Keeper of Dragon Magic
use super::prelude::*;

pub const ID: u32 = 48048590;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FUSION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Polymerization" Normal Spell or "Fusion" Normal Spell from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.AND(Cost.Discard(),s.cost))
    effect::set_cost(d, e1, Some(cost::and(vec![cost::discard(None, false, None, None, None), cost_target(cost)])));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon from your GY 1 of the Fusion Materials mentioned on a monster in the Extra Deck, in face-down Defense Position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(Cost.AND(s.spcost,s.cost))
    effect::set_cost(d, e3, Some(cost::and(vec![cost_target(spcost), cost_target(cost)])));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,function(c) return not (c:IsSummonLocation(LOCATION_EXTRA) and not c:IsType(TYPE_FUSION)) end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(activity_filter));
}
/// `function(c) return not (c:IsSummonLocation(LOCATION_EXTRA) and not c:IsType(TYPE_FUSION)) end`
fn activity_filter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(0);
    !(card::is_summon_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_FUSION))
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    // You cannot Special Summon monsters from the Extra Deck, except Fusion Monsters, the turn you activate either of this card's effects
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_FUSION) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_FUSION))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FUSION) and c:IsNormalSpell() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_FUSION as u16]) && card::is_normal_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.spcostfilter(c,e,tp)`
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_FUSION) and not c:IsPublic() and c.material
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,table.unpack(c.material))
    // (an empty list stands for a nil `c.material`: a Fusion Monster's script always lists its materials)
    let material = crate::card_info(d, c).material;
    card::is_type(d, c, TYPE_FUSION)
        && !card::is_public(d, c)
        && !material.is_empty()
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, material)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,...)`: `c:IsCode(...)` and it can be Special Summoned face-down
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, codes: &[u32]) -> bool {
    // return c:IsCode(...) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE)
    card::is_code(d, c, codes) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| spcostfilter(d, x, e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spcostfilter(d, x, e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g).nil();
    // Duel.ConfirmCards(1-tp,sc)
    duel::confirm_cards(d, 1 - tp, sc);
    // Duel.ShuffleExtra(tp)
    duel::shuffle_extra(d, tp);
    // e:SetLabel(table.unpack(sc.material))
    let labels: Vec<i64> = crate::card_info(d, sc).material.iter().map(|&m| m as i64).collect();
    effect::set_label(d, e, &labels);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // e:GetLabel() returns every label (0 if there is none) and they are the codes of the filter
    let mut labels: Vec<u32> = effect::get_labels(d, a.e).iter().map(|&l| l as u32).collect();
    if labels.is_empty() {
        labels.push(0);
    }
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,e:GetLabel())
    let e = a.e;
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp, &labels)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)>0 then
    if !g.is_empty() && duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
