//! Dark Calling
use super::prelude::*;

pub const ID: u32 = 12071500;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_FUSION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Fusion.CreateSummonEff{handler=c,fusfilter=s.fusfilter,matfilter=Fusion.InHandMat(Card.IsAbleToRemove),
    //     extrafil=s.fextra,extraop=Fusion.BanishMaterial,extratg=s.extratg,chkf=FUSPROC_NOLIMIT}
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| fusfilter(d, c))),
            matfilter: Some(fusion::in_hand_mat_with(fusion::mat_filter_fn(|d, c, _e, tp, chk| card::is_able_to_remove(d, c, None, Some(tp), Some(chk as u32))))),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extraop: Some(fusion::banish_material()),
            extratg: Some(cost_target(extratg)),
            chkf: Some(FUSPROC_NOLIMIT as u64),
            ..Default::default()
        },
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.fusfilter(c)`: `return c.dark_calling`
fn fusfilter(d: &mut Duel, c: CardId) -> bool {
    crate::card_info(d, c).dark_calling
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE|LOCATION_HAND,0,nil)
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)),
        tp,
        LOCATION_GRAVE | LOCATION_HAND,
        0,
        NO_EX,
    );
    (Some(g), None)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
