//! Roar of the Blue-Eyed Dragons
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 17725109;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLUE_EYES], listed_names: &[CARD_BLUEEYES_W_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Blue-Eyes" monster from your Deck, GY, or banishment
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Fusion Summon 1 Fusion Monster including a "Blue-Eyes" monster as material
    // local params={handler=c,extrafil=s.fmatextra}
    let params = fusion::SummonEff { extrafil: Some(std::sync::Arc::new(fmatextra)), ..Default::default() };
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(Fusion.SummonEffTG(params))
    let tg = fusion::summon_eff_tg(d, params.clone());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(params))
    let op = fusion::summon_eff_op(d, params);
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp,any)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, any: bool) -> bool {
    // return c:IsSetCard(SET_BLUE_EYES) and (any or c:IsCode(CARD_BLUEEYES_W_DRAGON))
    //   and (c:IsFaceup() or c:IsLocation(LOCATION_DECK))
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_set_card(d, c, &[SET_BLUE_EYES as u16])
        && (any || card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON]))
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_DECK))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `aux.FaceupFilter(Card.IsCode,CARD_BLUEEYES_W_DRAGON)`
fn faceup_bewd(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON])
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local any=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_BLUEEYES_W_DRAGON),tp,LOCATION_ONFIELD,0,1,nil)
    let any = duel::is_existing_matching_card(d, Some(&faceup_bewd), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp,any)
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| spfilter(d, c, a.e, a.tp, any)),
                a.tp,
                LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // You cannot Special Summon from the Extra Deck for the rest of this turn, except Dragon monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsRace(RACE_DRAGON) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_race(d, c, RACE_DRAGON))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // Clock Lizard check
    // aux.addTempLizardCheck(c,tp,function(e,c) return not c:IsOriginalRace(RACE_DRAGON) end)
    aux::add_temp_lizard_check(d, c, a.tp, Some(effect_card_fn(|d, _e, c| !card::is_original_race(d, c, RACE_DRAGON))), None, None, None, None);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local any=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_BLUEEYES_W_DRAGON),tp,LOCATION_ONFIELD,0,1,nil)
    let any = duel::is_existing_matching_card(d, Some(&faceup_bewd), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp,any)
    let nv = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp, any));
    let g = duel::select_matching_card(d, a.tp, Some(&nv), a.tp, LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
/// `s.fmatextra(e,tp,mg)`: `return nil,s.extramatcheck`
fn fmatextra(_d: &mut Duel, _e: EffectId, _tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    (None, Some(lua_fn(extramatcheck)))
}
/// `s.extramatcheck(tp,sg,fc)`: `return sg:IsExists(aux.FilterBoolFunction(Card.IsSetCard,SET_BLUE_EYES,fc,SUMMON_TYPE_FUSION,tp),1,nil)`
fn extramatcheck(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(0);
    let sg = a.g(1);
    let fc = a.c(2);
    group::is_exists(
        d,
        &sg,
        &|d, c| card::is_set_card_ctx(d, c, &[SET_BLUE_EYES as u16], Some(fc), SUMMON_TYPE_FUSION as u64, Some(tp)),
        1,
        NO_EX,
    )
}
