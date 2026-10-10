//! Primite Fusion
use super::prelude::*;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PRIMITE], ..ScriptInfo::DEFAULT };

pub const ID: u32 = 99161253;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon 1 Dragon Fusion Monster from your Extra Deck, by shuffling its materials from your field, GY, and/or banishment into the Deck, including a Normal Monster
    // local e1=Fusion.CreateSummonEff({handler=c,fusfilter=aux.FilterBoolFunction(Card.IsRace,RACE_DRAGON),matfilter=aux.FALSE,extrafil=s.fextra,extraop=Fusion.ShuffleMaterial,extratg=s.extratg})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_DRAGON))),
            matfilter: Some(fusion::mat_filter_fn(|_d, _c, _e, _tp, _chk| false)),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extraop: Some(fusion::shuffle_material()),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 Level 5 or higher "Primite" monster from your Deck or GY to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.fextrafil(c)`
fn fextrafil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToDeck() and (c:IsOnField() or c:IsFaceup())
    card::is_able_to_deck(d, c) && (card::is_on_field(d, c) || card::is_faceup(d, c))
}
/// `s.fcheck(tp,sg,fc)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    // return sg:IsExists(Card.IsType,1,nil,TYPE_NORMAL)
    let sg = a.g(1);
    group::is_exists(d, &sg, &|d, c| card::is_type(d, c, TYPE_NORMAL), 1, NO_EX)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(s.fextrafil),tp,LOCATION_ONFIELD|LOCATION_GRAVE|LOCATION_REMOVED,0,nil), s.fcheck
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_monster(d, c) && fextrafil(d, c)),
        tp,
        LOCATION_ONFIELD | LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        NO_EX,
    );
    (Some(g), Some(lua_fn(fcheck)))
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_ONFIELD|LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, (LOCATION_ONFIELD | LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(5) and c:IsSetCard(SET_PRIMITE) and c:IsAbleToHand()
    card::is_level_above(d, c, 5) && card::is_set_card(d, c, &[SET_PRIMITE as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let nf = aux::necro_valley_filter(thfilter);
    let g = duel::select_matching_card(d, tp, Some(&nf), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
