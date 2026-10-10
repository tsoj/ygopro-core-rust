//! Ultimate Fusion
use super::prelude::*;

pub const ID: u32 = 71143015;
const CARD_BLUEEYES_U_DRAGON: u32 = 23995346;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_BLUEEYES_W_DRAGON, CARD_BLUEEYES_U_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff({handler=c,fusfilter=s.ffilter,matfilter=Card.IsAbleToDeck,
    //   extrafil=s.fextra,extraop=Fusion.ShuffleMaterial,stage2=s.desop,extratg=s.extrtarget})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| ffilter(d, c))),
            matfilter: Some(fusion::mat_filter_fn(|d, c, _e, _tp, _chk| card::is_able_to_deck(d, c))),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extraop: Some(fusion::shuffle_material()),
            stage2: Some(std::sync::Arc::new(desop)),
            extratg: Some(cost_target(extrtarget)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.ffilter(c)`
fn ffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:ListsCodeAsMaterial(CARD_BLUEEYES_W_DRAGON,CARD_BLUEEYES_U_DRAGON)
    card::lists_code_as_material(d, c, &[CARD_BLUEEYES_W_DRAGON, CARD_BLUEEYES_U_DRAGON])
}
/// `s.fextra(e,tp,mg,sumtype)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(aux.NecroValleyFilter(Fusion.IsMonsterFilter(Card.IsAbleToDeck)),tp,LOCATION_GRAVE,0,nil),s.fcheck
    let nf = aux::necro_valley_filter(|d, c| card::is_monster(d, c) && card::is_able_to_deck(d, c));
    let g = duel::get_matching_group(d, Some(&nf), tp, LOCATION_GRAVE, 0, NO_EX);
    (Some(g), Some(lua_fn(fcheck)))
}
/// `s.fcheck(tp,sg,fc,sumtype,tp)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    // return sg:IsExists(s.forcedmatfilter,1,nil,fc,sumtype,tp)
    let sg = a.g(1);
    let fc = a.c(2);
    let sumtype = a.int(3) as u64;
    let tp = a.p(4);
    group::is_exists(d, &sg, &|d, c| forcedmatfilter(d, c, fc, sumtype, tp), 1, NO_EX)
}
/// `s.forcedmatfilter(c,fc,sumtype,tp)`
fn forcedmatfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsCode(CARD_BLUEEYES_W_DRAGON,CARD_BLUEEYES_U_DRAGON) and c:IsSummonCode(fc,sumtype,tp,fc.material)
    let material = crate::class::material(d, fc);
    card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON, CARD_BLUEEYES_U_DRAGON]) && card::is_summon_code(d, c, Some(fc), sumtype, tp, &material)
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // local code=c:GetPreviousCodeOnField()
    let code = card::get_previous_code_on_field(d, c).first().copied();
    // return (code==CARD_BLUEEYES_W_DRAGON or code==CARD_BLUEEYES_U_DRAGON) and c:IsPreviousLocation(LOCATION_ONFIELD)
    (code == Some(CARD_BLUEEYES_W_DRAGON) || code == Some(CARD_BLUEEYES_U_DRAGON)) && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
/// `s.desop(e,tc,tp,mg,chk)`
fn desop(d: &mut Duel, _e: EffectId, _tc: Option<CardId>, tp: u8, mg: Option<&CardSet>, chk: i64) {
    // if chk==0 then
    if chk != 0 {
        return;
    }
    let mg = mg.nil();
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // local mc=mg:FilterCount(s.desfilter,nil)
    let mc = group::filter_count(d, mg, &desfilter, NO_EX);
    // if #g>0 and mc>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !g.is_empty() && mc > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local sg=g:Select(tp,1,mc,nil)
        let sg = group::select(d, &g, tp, 1, mc as u16, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(sg,REASON_EFFECT)
            duel::destroy(d, sg, REASON_EFFECT, None, None);
        }
    }
}
/// `s.extrtarget(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extrtarget(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,0,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 0, a.tp, (LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,0,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 0, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
